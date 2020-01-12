use std::ops::{Mul, Deref, DerefMut};
use cgmath::{Matrix4, One, Point3, Vector3, InnerSpace, Rad};

///
/// A transform that can be pushed onto a transformation
/// state. Each of the elements in the last transformation
/// will be multiplied with their corresponding elements
/// here.
///
/// This is `Copy` + `Clone` where `T: Copy` and `T: Clone`
/// respectively. A `Transform` is meant to be owned by
/// each object so as to allow each object to transform
/// itself and pass a copy of it when rendering.
///
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct Transform<T> {
    ///
    /// The scale matrix which will be pushed onto the
    /// current state.
    ///
    pub scale: T,
    ///
    /// The rotation matrix which will be pushed onto the
    /// current state.
    ///
    pub rotate: T,
    ///
    /// The translation matrix which will be pushed onto
    /// the current state.
    ///
    pub translate: T,
}

impl One for Transform<Matrix4<f32>> {
    fn one() -> Self {
        Self {
            scale: One::one(),
            rotate: One::one(),
            translate: One::one(),
        }
    }
}

impl Mul<Self> for Transform<Matrix4<f32>> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Transform {
            scale: self.scale * rhs.scale,
            rotate: self.rotate * rhs.rotate,
            translate: self.translate * rhs.translate,
        }
    }
}

impl Transform<Matrix4<f32>> {
    ///
    /// An identity transform. Does nothing when applied
    ///
    #[inline]
    pub fn identity() -> Self {
        One::one()
    }

    ///
    /// Translates this transform by `delta_pos` relative
    /// to its local origin (Before rotations).
    ///
    #[inline]
    pub fn translate_by(&mut self, delta_pos: Vector3<f32>) {
        self.translate = self.translate * Matrix4::from_translation(delta_pos);
    }
    ///
    /// Translates `magnitude * |direction|` units in the
    /// direction represented by the direction vector relative
    /// to the origin.
    ///
    #[inline]
    pub fn translate_dir(&mut self, direction: Vector3<f32>, magnitude: f32) {
        self.translate_by(direction * magnitude);
    }
    ///
    /// Translates `magnitude` units in the direction from `a`
    /// to `b`.
    ///
    /// Subtracts `a` from `b`, normalizes, and translates `distance`
    /// units relative to itself in local space.
    ///
    #[inline]
    pub fn translate_dir_2_points(&mut self, point_a: Point3<f32>, point_b: Point3<f32>, distance: f32) {
        self.translate_dir(Vector3::normalize(point_b - point_a), distance)
    }

    ///
    /// Rotates this transform along an axis relative to the origin by
    /// `angle` radians.
    ///
    #[inline]
    pub fn rotate_axis(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
        self.rotate = self.rotate * Matrix4::from_axis_angle(axis, angle);
    }
    ///
    /// Rotates this transform using a preexisting rotation transform.
    ///
    #[inline]
    pub fn rotate_preexisting<T: Into<Matrix4<f32>>>(&mut self, value: T) {
        self.rotate = self.rotate * value.into();
    }
    ///
    /// Rotates this transform to look at a target.
    ///
    #[inline]
    pub fn rotate_look_at_target(&mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) {
        self.rotate = self.rotate * Matrix4::look_at(eye, target, up);
    }
    ///
    /// Rotates this transform to look in a direction relative to the origin.
    ///
    #[inline]
    pub fn rotate_look_at_dir(&mut self, eye: Point3<f32>, direction: Vector3<f32>, up: Vector3<f32>) {
        self.rotate = self.rotate * Matrix4::look_at_dir(eye, direction, up);
    }

    ///
    /// Scales this transform's x.
    ///
    #[inline]
    pub fn scale_x(&mut self, amount: f32) {
        self.scale = self.scale * Matrix4::from_nonuniform_scale(amount, 1., 1.);
    }
    ///
    /// Scales this transform's y.
    ///
    #[inline]
    pub fn scale_y(&mut self, amount: f32) {
        self.scale = self.scale * Matrix4::from_nonuniform_scale(1., amount, 1.);
    }
    ///
    /// Scales this transform's z.
    ///
    #[inline]
    pub fn scale_z(&mut self, amount: f32) {
        self.scale = self.scale * Matrix4::from_nonuniform_scale(1., 1., amount);
    }
    ///
    /// Uniformly scales this transform.
    ///
    #[inline]
    pub fn scale(&mut self, amount: f32) {
        self.scale = self.scale * Matrix4::from_scale(amount);
    }
    ///
    /// Scales this transform's x, y, and z.
    ///
    #[inline]
    pub fn scale_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.scale = self.scale * Matrix4::from_nonuniform_scale(x, y, z);
    }
}

///
/// This is a lock on a pushed transform which will automatically
/// pop the transform it was created from on drop.
///
#[must_use = "if the lock is immediately dropped, the transform will never be used!"]
pub enum TransformLock<'a, T: Clone + Mul<Output=T>, F: Fn(T, T, T) -> T> {
    With {
        lock: &'a mut TransformHierarchy<T, F>,
        index: usize,
    },
    Without {
        lock: &'a mut TransformHierarchy<T, F>,
        index: usize,
    }
}

impl<'a, T: Clone + Mul<Output=T>, F: Fn(T, T, T) -> T> Drop for TransformLock<'a, T, F> {
    fn drop(&mut self) {
        if let TransformLock::With {lock, ..} = self {
            lock.pop_one();
        }
    }
}

impl<'a, T: Clone + Mul<Output=T>, F: Fn(T, T, T) -> T> TransformLock<'a, T, F> {
    ///
    /// Gets the transform the lock is holding.
    ///
    pub fn current(&'_ self) -> &'_ T {
        match self {
            TransformLock::With {lock, index} |
            TransformLock::Without {lock, index} => {
                &lock.matrices[*index]
            }
        }
    }
}

impl<'a, T: Clone + Mul<Output=T>, F: Fn(T, T, T) -> T> Deref for TransformLock<'a, T, F> {
    type Target = TransformHierarchy<T, F>;
    fn deref(&self) -> &Self::Target {
        match self {
            TransformLock::With {lock, ..} |
            TransformLock::Without {lock, ..} => {
                &**lock
            }
        }
    }
}

impl<'a, T: Clone + Mul<Output=T>, F: Fn(T, T, T) -> T> DerefMut for TransformLock<'a, T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            TransformLock::With { lock, .. } |
            TransformLock::Without { lock, .. } => {
                &mut**lock
            }
        }
    }
}

///
/// A Binary-tree traversal method of storing transformations.
///
/// The latest pushed transformation will be the first to be
/// applied to a point since it is the most local space to that
/// point. The first transformation (The identity in the case
/// of a lack of them) will be the last to be applied to a point.
///
/// This assumes the point operations will happen like so:
/// `new_point = projection * view * world * point`.
///
#[derive(Debug, Clone)]
pub struct TransformHierarchy<T: Clone + Mul<Output = T>, F: Fn(T, T, T) -> T> {
    matrices: Vec<T>,
    order_func: F,
}

impl<T: Clone + Mul<Output = T>, F: Fn(T, T, T) -> T> TransformHierarchy<T, F> {
    pub fn new(identity: T, func: F) -> Self {
        Self {
            order_func: func,
            matrices: vec![identity]
        }
    }

    pub fn push(&'_ mut self, push_scale: T, push_rotate: T, push_translate: T) -> TransformLock<'_, T, F> {
        let previous = self.matrices.last()
            .cloned()
            .unwrap();
        let new_transform = previous * (self.order_func)(
            push_scale, push_rotate, push_translate
        );
        let len = self.matrices.len();
        self.matrices.push(new_transform.clone());
        TransformLock::With {
            lock: self,
            index: len
        }
    }

    #[inline]
    pub fn push_transform(&'_ mut self, Transform {scale, rotate, translate}: Transform<T>) -> TransformLock<'_, T, F> {
        self.push(scale, rotate, translate)
    }

    pub fn push_none(&'_ mut self) -> TransformLock<'_, T, F> {
        let len = self.matrices.len() - 1;
        TransformLock::Without {
            lock: self,
            index: len,
        }
    }

    pub(crate) fn pop_one(&mut self) -> T {
        if self.matrices.len() > 1 {
            let val = self.matrices.pop();

            val.unwrap()
        } else {
            panic!("Cannot pop past the first element: The identity.");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TransformHierarchy;

    #[test]
    fn identity() {
        let mut transform = TransformHierarchy::<f32, _>::new(1f32, |x, y, z| x * y * z);
        let lock = transform.push(1., 1., 1.);
    }

    #[test]
    fn push() {
        let mut transform = TransformHierarchy::<f32, _>::new(1f32, |x, y, z| x * y * z);
        let mut first = transform.push(100., -3., std::f32::consts::SQRT_2);
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0b95b1841ab0090cbd81ee16b394f6cb
        assert_eq!(*first.current(), -424.26406860351562500000);
        let second = first.push(0., 1., 1.);
        assert_eq!(*second.current(), 0.);
    }

    #[test]
    #[should_panic]
    fn pop_one_none() {
        let mut transform = TransformHierarchy::<f32, _>::new(1f32, |x, y, z| x * y * z);
        transform.pop_one();
    }

    #[test]
    fn pop_one() {
        let mut transform = TransformHierarchy::<f32, _>::new(1f32, |x, y, z| x * y * z);
        let x = transform.push(2., 3., 4.);
        std::mem::forget(x);
        assert_eq!(transform.pop_one(), 2. * 3. * 4.);
    }
}
