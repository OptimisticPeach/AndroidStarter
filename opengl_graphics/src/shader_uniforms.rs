//! Types and methods for setting shader uniforms


// External crates.
use std::ffi::CString;
use gl;
use gl::types::{GLboolean, GLint, GLuint};
use std::marker::PhantomData;

// Local crate.
use back_end::GlGraphics;

/// Describes a shader uniform of a given type.
#[derive(Clone, Copy)]
pub struct ShaderUniform<T : ?Sized>{
    location : GLint,
    phantom : PhantomData<T>,
}

/// Shader uniform type
///
/// For now a small subset
pub trait UniformType<'a> {
    /// The value given to the uniform
    type Value: 'a;
    /// Sets the uniform to a value
    fn set(Self::Value, location: GLint, program: GLuint);
}

/// Shader uniform float
#[derive(Clone, Copy)]
pub struct SUFloat {}
impl UniformType<'_> for SUFloat {
    type Value = f32;
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform1f(p, location, value)}
    }
}

/// Shader uniform integer
#[derive(Clone, Copy)]
pub struct SUInt {}
impl UniformType<'_> for SUInt {
    type Value = i32;
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform1i(p, location, value)}
    }
}

/// Shader uniform integer
#[derive(Clone, Copy)]
pub struct SUUInt3 {}
impl UniformType<'_> for SUUInt3 {
    type Value = [u32; 3];
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform3ui(p, location, value[0], value[1], value[2])}
    }
}

/// Shader uniform vector of size 2
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec2 {}
impl<'a> UniformType<'a> for SUVec2 {
    type Value = &'a [f32; 2];
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform2f(p, location, value[0], value[1])}
    }
}

/// Shader uniform vector of size 3
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec3 {}
impl<'a> UniformType<'a> for SUVec3 {
    type Value = &'a [f32; 3];
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform3f(p, location, value[0], value[1], value[2])}
    }
}

/// Shader uniform vector of size 4
/// Vector elements are floats
#[derive(Clone, Copy)]
pub struct SUVec4 {}
impl<'a> UniformType<'a> for SUVec4 {
    type Value = &'a [f32; 4];
    fn set(value: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniform4f(p, location, value[0], value[1], value[2], value[3])}
    }
}

/// Shader uniform 2x2 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat2x2 {}
impl<'a> UniformType<'a> for SUMat2x2 {
    type Value = &'a [f32; 4];
    fn set(values: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniformMatrix2fv(p, location, 1 as GLint, false as GLboolean, values.as_ptr())}
    }
}

/// Shader uniform 3x3 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat3x3 {}
impl<'a> UniformType<'a> for SUMat3x3 {
    type Value = &'a [f32; 9];
    fn set(values: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniformMatrix3fv(p, location, 1 as GLint, false as GLboolean, values.as_ptr())}
    }
}

/// Shader uniform 4x4 matrix
/// Matrix elements are floats
#[derive(Clone, Copy)]
pub struct SUMat4x4 {}
impl<'a> UniformType<'a> for SUMat4x4 {
    type Value = &'a [f32; 16];
    fn set(values: Self::Value, location: GLint, p: GLuint) {
        unsafe {gl::ProgramUniformMatrix4fv(p, location, 1 as GLint, false as GLboolean, values.as_ptr())}
    }
}

impl GlGraphics {
    /// Try to get uniform from the current shader of a given name.
    pub fn get_uniform<T: ?Sized>(&self, name : &str) -> Option<ShaderUniform<T>> where for<'a> T: UniformType<'a> {
        self.get_current_program().and_then( |p| {
            unsafe {
                let c_source = CString::new(name).ok();
                c_source.and_then(|name| {
                    let uniform = match gl::GetUniformLocation(p, name.as_ptr()) {
                        -1 => None,
                        location => {
                            Some(ShaderUniform{
                                location : location,
                                phantom : PhantomData,
                            })
                        },
                    };
                    drop(name);
                    uniform
                })
            }
        })
    }
}

impl<'a, T> ShaderUniform<T> where T: UniformType<'a> {
    /// Set the value of the float uniform.
    pub fn set(&self, gl : &GlGraphics, value: T::Value) {
        gl.get_current_program().map(|p| {
            T::set(value, self.location, p);
        });
    }
}
