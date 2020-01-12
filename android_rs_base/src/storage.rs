use std::collections::HashMap;
use opengl_graphics::shader_utils::Shader;
use std::any::{TypeId, Any};
use opengl_graphics::{GLSL, GlGraphics};
use graphics::Context;
use piston::input::RenderArgs;
use cgmath::{Matrix4, SquareMatrix, Vector3, Quaternion, Rotation3, Rad, Transform as Transformation, Point3, EuclideanSpace};
use matrices::{TransformHierarchy, Transform as BasicTransform};

pub type Transforms = TransformHierarchy<Matrix4<f32>, fn(Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) -> Matrix4<f32>>;
pub type Transform = BasicTransform<Matrix4<f32>>;

pub struct ShaderStorage {
    shaders: HashMap<TypeId, Box<dyn Any>>,
    pub cache: ViewProj
}

pub struct ViewProj {
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

impl Default for ViewProj {
    fn default() -> Self {
        Self {
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
        }
    }
}

impl ViewProj {
    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }
    pub fn projection(&self) -> Matrix4<f32> {
        self.projection
    }
    pub fn eye(&self) -> Point3<f32> {
        self.view.transform_point(Point3::origin())
    }

    pub fn view_ref(&self) -> &[f32; 16] {
        self.view.as_ref()
    }
    pub fn projection_ref(&self) -> &[f32; 16] {
        self.projection.as_ref()
    }

    pub fn rotate_view_axis_angle(&mut self, axis: Vector3<f32>, angle: f32) {
        self.view = self.view * Matrix4::from(Quaternion::from_axis_angle(axis, Rad(-angle)));
    }
    pub fn translate_view(&mut self, delta: Vector3<f32>) {
        self.view = self.view * Matrix4::from_translation(-delta);
    }
    pub fn set_view_pos(&mut self, pos: Vector3<f32>) {
        self.view = Matrix4::from_translation(-pos);
    }
    pub fn set_projection(&mut self, projection: Matrix4<f32>) {
        self.projection = projection;
    }
}

impl ShaderStorage {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            cache: ViewProj::default(),
        }
    }

    pub fn get<T: Any + Shader>(&mut self, gl: GLSL, graphics: &mut GlGraphics) -> (&mut T, &mut ViewProj) {
        (
            (
                &mut **
                    self
                        .shaders
                        .entry(TypeId::of::<T>())
                        .or_insert_with(|| Box::new(T::new(gl, Some(graphics))) as Box<_>)
            ).downcast_mut().unwrap(),
            &mut self.cache,
        )
    }
}

pub trait Drawable {
    type Shader: Shader;
    fn draw_with(
        &mut self,
        data: &mut Self::Shader,
        graphics: &mut GlGraphics,
        context: &Context,
        cache: &mut ViewProj,
        transforms: &mut Transforms
    );

    #[allow(unused_variables)]
    fn draw_children(&mut self, context: &mut ShaderContext) {}
    #[allow(unused_variables)]
    fn prepare_draw(&mut self,
                    data: &mut Self::Shader,
                    cache: &mut ViewProj,
                    transforms: &mut Transforms) {}
}

pub struct ShaderContext<'a, 'b> {
    pub shaders: &'a mut ShaderStorage,
    pub gl: &'b mut GlGraphics,
    pub c: Context,
    pub rargs: RenderArgs,
    pub transforms: Transforms,
}

impl<'a, 'b> ShaderContext<'a, 'b> {
    pub fn new(s: &'a mut ShaderStorage, gl: &'b mut GlGraphics, c: Context, rargs: RenderArgs) -> Self {
        Self {
            gl,
            c,
            shaders: s,
            rargs,
            transforms: TransformHierarchy::new(Matrix4::identity(), |s, r, t| s * r * t),
        }
    }
    pub fn draw<T: Drawable>(&mut self, item: &mut T) where T::Shader: Any {
        let (
            shader,
            mats
        ) = self.shaders.get::<T::Shader>(GLSL::V1_20, &mut self.gl);
        item.prepare_draw(shader, mats, &mut self.transforms);
        item.draw_with(
            shader,
            &mut self.gl,
            &self.c,
            mats,
            &mut self.transforms,
        );
        item.draw_children(self);
    }
}
