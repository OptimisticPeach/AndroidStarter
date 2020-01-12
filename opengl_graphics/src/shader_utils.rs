//! Helper functions for dealing with shaders.

// External crates.
use gl;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLsizeiptr, GLuint};
use shader_version::glsl::GLSL;
use std::ffi::CString;
use std::{ptr, mem};
use std::marker::PhantomData;

/// Vertices attributes
pub unsafe trait VertexAttribute: Copy {
    /// GL type.
    const TY: GLenum;
    /// Number of components
    const SIZE: i32;
}

unsafe impl VertexAttribute for f32 {
    const TY: GLenum = gl::FLOAT;
    const SIZE: i32 = 1;
}

unsafe impl VertexAttribute for [f32; 2] {
    const TY: GLenum = gl::FLOAT;
    const SIZE: i32 = 2;
}

unsafe impl VertexAttribute for [f32; 3] {
    const TY: GLenum = gl::FLOAT;
    const SIZE: i32 = 3;
}

unsafe impl VertexAttribute for [f32; 4] {
    const TY: GLenum = gl::FLOAT;
    const SIZE: i32 = 4;
}

/// Describes a shader attribute.
pub struct DynamicAttribute<T: VertexAttribute> {
    /// The vertex buffer object.
    pub(self) vbo: GLuint,
    /// The location of the attribute in shader.
    pub(self) location: GLuint,
    /// Whether to normalize when sending to GPU.
    normalize: GLboolean,
    /// Phantom
    phantom: PhantomData<T>,
}

impl<T: VertexAttribute> Drop for DynamicAttribute<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl<T: VertexAttribute> DynamicAttribute<T> {
    /// Binds to a vertex array object.
    ///
    /// The vertex array object remembers the format for later.
    pub fn bind_vao(&self, vao: GLuint) {
        let stride = 0;
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(self.location,
                                    T::SIZE,
                                    T::TY,
                                    self.normalize,
                                    stride,
                                    ptr::null());
        }
    }

    /// Creates new dynamic attribute
    pub fn new(program: GLuint,
           name: &str,
           normalize: GLboolean)
           -> Result<Self, String> {
        let location = attribute_location(program, name)?;
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        let res = DynamicAttribute {
            vbo: vbo,
            location: location,
            normalize: normalize,
            phantom: PhantomData,
        };
        Ok(res)
    }
    
    /// Sets attribute data.
    pub unsafe fn set(&self, data: &[T]) {
        gl::EnableVertexAttribArray(self.location);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       data.len() as GLsizeiptr * mem::size_of::<T>() as GLsizeiptr,
                       mem::transmute(data.as_ptr()),
                       gl::DYNAMIC_DRAW);
    }
}

impl DynamicAttribute<[f32; 4]> {
    /// Create XYZW vertex attribute.
    pub fn xyzw(program: GLuint, name: &str) -> Result<Self, String> {
        Self::new(program, name, gl::FALSE)
    }

    /// Create RGBA color attribute.
    pub fn rgba(program: GLuint, name: &str) -> Result<Self, String> {
        Self::new(program, name, gl::FALSE)
    }
}

impl DynamicAttribute<[f32; 3]> {
    /// Create XYZ vertex attribute.
    pub fn xyz(program: GLuint, name: &str) -> Result<Self, String> {
        Self::new(program, name, gl::FALSE)
    }

    /// Create RGB color attribute.
    pub fn rgb(program: GLuint, name: &str) -> Result<Self, String> {
        DynamicAttribute::new(program, name, gl::FALSE)
    }
}

impl DynamicAttribute<[f32; 2]> {

    /// Create texture coordinate attribute.
    pub fn uv(program: GLuint, name: &str) -> Result<Self, String> {
        DynamicAttribute::new(program, name, gl::FALSE)
    }

    /// Create XY vertex attribute.
    pub fn xy(program: GLuint, name: &str) -> Result<Self, String> {
        DynamicAttribute::new(program, name, gl::FALSE)
    }
}

impl DynamicAttribute<f32> {
    /// Create floating point attribute.
    pub fn f(program: GLuint, name: &str) -> Result<Self, String> {
        DynamicAttribute::new(program, name, gl::FALSE)
    }
}

/// An instanced attribute
pub struct InstancedAttribute<T: VertexAttribute> {
    dynamic_attribute: DynamicAttribute<T>,
    divisor: GLuint,
}

impl<T: VertexAttribute> InstancedAttribute<T> {
    /// Creates instanced attribute from preexisting dynamic attribute
    pub fn from_dynamic_attr(attribute: DynamicAttribute<T>) -> Self {
        Self {
            dynamic_attribute: attribute,
            divisor: 0
        }
    }

    /// Sets the data in this instanced attribute
    pub unsafe fn set(&mut self, data: &[T]) {
        gl::EnableVertexAttribArray(self.dynamic_attribute.location);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.dynamic_attribute.vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       data.len() as GLsizeiptr * mem::size_of::<T>() as GLsizeiptr,
                       data as *const [T] as *const T as *const std::ffi::c_void,
                       gl::DYNAMIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    /// Sets the divisor for this instanced attribute
    pub unsafe fn divisor(&mut self, divisor: GLuint) {
        self.divisor = divisor;
    }

    /// Binds to vao.
    pub unsafe fn bind_vao(&mut self, vao: GLuint) {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.dynamic_attribute.vbo);
        gl::VertexAttribPointer(self.dynamic_attribute.location,
                                T::SIZE, 
                                T::TY, 
                                self.dynamic_attribute.normalize,
                                0,
                                std::ptr::null_mut());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::VertexAttribDivisor(self.dynamic_attribute.location, self.divisor);
        
    }
}

/// Compiles a shader.
///
/// Returns a shader or a message with the error.
pub fn compile_shader(shader_type: GLenum, source: &str) -> Result<GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_source = match CString::new(source) {
            Ok(x) => x,
            Err(err) => return Err(format!("compile_shader: {}", err)),
        };
        gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
        drop(source);
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status == (gl::TRUE as GLint) {
            Ok(shader)
        } else {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            if len == 0 {
                Err("Compilation failed with no log. \
                     The OpenGL context might have been created on another thread, \
                     or not have been created."
                    .to_string())
            } else {
                // Subtract 1 to skip the trailing null character.
                let mut buf = vec![0; len as usize - 1];
                gl::GetShaderInfoLog(shader,
                                     len,
                                     ptr::null_mut(),
                                     buf.as_mut_ptr() as *mut GLchar);

                gl::DeleteShader(shader);

                Err(String::from_utf8(buf).ok().expect("ShaderInfoLog not valid utf8"))
            }
        }
    }
}

/// Finds attribute location from a program.
///
/// Returns `Err` if there is no attribute with such name.
pub fn attribute_location(program: GLuint, name: &str) -> Result<GLuint, String> {
    unsafe {
        let c_name = match CString::new(name) {
            Ok(x) => x,
            Err(err) => return Err(format!("attribute_location: {}", err)),
        };
        let id = gl::GetAttribLocation(program, c_name.as_ptr());
        drop(c_name);
        if id < 0 {
            Err(format!("Attribute '{}' does not exists in shader", name))
        } else {
            Ok(id as GLuint)
        }
    }
}

/// Finds uniform location from a program.
///
/// Returns `Err` if there is no uniform with such name.
pub fn uniform_location(program: GLuint, name: &str) -> Result<GLuint, String> {
    unsafe {
        let c_name = match CString::new(name) {
            Ok(x) => x,
            Err(err) => return Err(format!("uniform_location: {}", err)),
        };
        let id = gl::GetUniformLocation(program, c_name.as_ptr());
        drop(c_name);
        if id < 0 {
            Err(format!("Uniform '{}' does not exists in shader", name))
        } else {
            Ok(id as GLuint)
        }
    }
}

///
/// Generic shader trait. Don't forget to impl Drop.
/// 
pub trait Shader {
    /// The type of vertex; [f32; 2], [f32; 3] or [f32; 4];
    type Vertex: Copy;
    /// Creates a new instance of this shader. (Includes compilation)
    fn new(glsl: GLSL, gl: Option<&mut crate::back_end::GlGraphics>) -> Self where Self: Sized;
    /// Flushes values to the gpu and draws them
    fn flush(&mut self);
    /// Gets the program for this shader
    fn program(&self) -> GLuint;
    /// Gets the offset of the vertices currently buffered
    fn offset(&mut self) -> &mut usize;
    /// Gets a mutable reference to the position buffer
    fn pos_buffer(&mut self) -> &mut Vec<Self::Vertex>;
    /// Optionally gets a mutable reference to the colour buffer if supported
    fn colour_buffer(&mut self) -> Option<&mut Vec<[f32; 4]>> { None }
    /// Optionally gets a mutable reference to the uv buffer if supported
    fn uv_buffer(&mut self) -> Option<&mut Vec<[f32; 2]>> { None }
    /// Optionally gets a mutable reference to the index buffer if supported
    fn index_buffer(&mut self) -> Option<&mut Vec<u16>> { None }
    /// Optionally gets a mutable reference to the normal buffer if supported
    fn normal_buffer(&mut self) -> Option<&mut Vec<[f32; 3]>> { None }
    /// Optionally gets a mutable reference to the texture id if supported
    fn texture_id(&mut self) -> Option<&mut GLuint> { None }
    /// Returns if it supports a texture
    fn has_texture(&self) -> bool { false }
}

macro_rules! unit_unimplemented_panic {
    () => {
        panic!("() is not a valid shader.")
    };
}

impl Shader for () {
    type Vertex = ();
    fn new(_glsl: GLSL, _gl: Option<&mut crate::back_end::GlGraphics>) -> Self where Self: Sized {
        ()
    }
    fn flush(&mut self) {
        unit_unimplemented_panic!();
    }
    fn program(&self) -> GLuint {
        unit_unimplemented_panic!();
    }
    fn offset(&mut self) -> &mut usize {
        unit_unimplemented_panic!();
    }
    fn pos_buffer(&mut self) -> &mut Vec<Self::Vertex> {
        unit_unimplemented_panic!();
    }
}
