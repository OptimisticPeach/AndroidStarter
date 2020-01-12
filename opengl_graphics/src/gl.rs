
        mod __gl_imports {
            pub use std::mem;
            pub use std::os::raw;
        }
    

        #[inline(never)]
        fn metaloadfn(mut loadfn: &mut FnMut(&str) -> *const __gl_imports::raw::c_void,
                      symbol: &str,
                      fallbacks: &[&str]) -> *const __gl_imports::raw::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in fallbacks {
                    ptr = loadfn(sym);
                    if !ptr.is_null() { break; }
                }
            }
            ptr
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = extern "system" fn(source: GLenum,
                                          gltype: GLenum,
                                          id: GLuint,
                                          severity: GLenum,
                                          length: GLsizei,
                                          message: *const GLchar,
                                          userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint,
                                             category: GLenum,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;


        }
    
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_PROGRAM: types::GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_RESOURCES: types::GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_VARIABLES: types::GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_POINT_SIZE_RANGE: types::GLenum = 0x846D;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_BARRIER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_SHADER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BITS: types::GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)] pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED_CONSERVATIVE: types::GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_SIZE: types::GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_STRIDE: types::GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BARRIER_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER: types::GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_BINDING: types::GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_START: types::GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLOCK_INDEX: types::GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BITS: types::GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_BINDING: types::GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_DATA_SIZE: types::GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_UPDATE_BARRIER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_VARIABLE: types::GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLORBURN: types::GLenum = 0x929A;
#[allow(dead_code, non_upper_case_globals)] pub const COLORDODGE: types::GLenum = 0x9299;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMMAND_BARRIER_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_R11_EAC: types::GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG11_EAC: types::GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_ETC2: types::GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA8_ETC2_EAC: types::GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_10x10: types::GLenum = 0x93BB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_10x5: types::GLenum = 0x93B8;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_10x6: types::GLenum = 0x93B9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_10x8: types::GLenum = 0x93BA;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_12x10: types::GLenum = 0x93BC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_12x12: types::GLenum = 0x93BD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_4x4: types::GLenum = 0x93B0;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_5x4: types::GLenum = 0x93B1;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_5x5: types::GLenum = 0x93B2;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_6x5: types::GLenum = 0x93B3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_6x6: types::GLenum = 0x93B4;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_8x5: types::GLenum = 0x93B5;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_8x6: types::GLenum = 0x93B6;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_ASTC_8x8: types::GLenum = 0x93B7;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_R11_EAC: types::GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG11_EAC: types::GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: types::GLenum = 0x93DB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: types::GLenum = 0x93D8;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: types::GLenum = 0x93D9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: types::GLenum = 0x93DA;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: types::GLenum = 0x93DC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: types::GLenum = 0x93DD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: types::GLenum = 0x93D0;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: types::GLenum = 0x93D1;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: types::GLenum = 0x93D2;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: types::GLenum = 0x93D3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: types::GLenum = 0x93D4;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: types::GLenum = 0x93D5;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: types::GLenum = 0x93D6;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: types::GLenum = 0x93D7;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: types::GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ETC2: types::GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER: types::GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_LOST: types::GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER_BINDING: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER_BINDING: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DARKEN: types::GLenum = 0x9297;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BITS: types::GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_TEXTURE_MODE: types::GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DIFFERENCE: types::GLenum = 0x929E;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER: types::GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER_BINDING: types::GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER: types::GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER_BINDING: types::GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BARRIER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EXCLUSION: types::GLenum = 0x92A0;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_EVEN: types::GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_ODD: types::GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: types::GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BARRIER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_HEIGHT: types::GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_LAYERS: types::GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_SAMPLES: types::GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_WIDTH: types::GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: types::GLenum = 0x8CD9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GENERATE_MIPMAP_HINT: types::GLenum = 0x8192;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BITS: types::GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)] pub const GUILTY_CONTEXT_RESET: types::GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)] pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)] pub const HARDLIGHT: types::GLenum = 0x929B;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_FLOAT: types::GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_INT: types::GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)] pub const HSL_COLOR: types::GLenum = 0x92AF;
#[allow(dead_code, non_upper_case_globals)] pub const HSL_HUE: types::GLenum = 0x92AD;
#[allow(dead_code, non_upper_case_globals)] pub const HSL_LUMINOSITY: types::GLenum = 0x92B0;
#[allow(dead_code, non_upper_case_globals)] pub const HSL_SATURATION: types::GLenum = 0x92AE;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D: types::GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_ARRAY: types::GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_3D: types::GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_ACCESS: types::GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_FORMAT: types::GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYER: types::GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYERED: types::GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LEVEL: types::GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_NAME: types::GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BUFFER: types::GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE: types::GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: types::GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: types::GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: types::GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_FORMAT: types::GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_TYPE: types::GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INNOCENT_CONTEXT_RESET: types::GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D: types::GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_ARRAY: types::GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_3D: types::GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_BUFFER: types::GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE: types::GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const ISOLINES: types::GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)] pub const IS_PER_PATCH: types::GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)] pub const IS_ROW_MAJOR: types::GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)] pub const LAYER_PROVOKING_VERTEX: types::GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTEN: types::GLenum = 0x9298;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION: types::GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET: types::GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_FLOAT: types::GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_INT: types::GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE: types::GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_ALPHA: types::GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)] pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_STRIDE: types::GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: types::GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTERS: types::GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNIFORMS: types::GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTERS: types::GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_IMAGE_UNIFORMS: types::GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: types::GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: types::GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_BLOCKS: types::GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_COUNT: types::GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: types::GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENT_INDEX: types::GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTERS: types::GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_IMAGE_UNIFORMS: types::GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_VECTORS: types::GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_HEIGHT: types::GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_LAYERS: types::GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_SAMPLES: types::GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_WIDTH: types::GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTERS: types::GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_IMAGE_UNIFORMS: types::GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_UNITS: types::GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_LENGTH: types::GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_ACTIVE_VARIABLES: types::GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PATCH_VERTICES: types::GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BLOCK_SIZE: types::GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: types::GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: types::GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: types::GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: types::GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: types::GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: types::GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: types::GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: types::GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: types::GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: types::GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: types::GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_GEN_LEVEL: types::GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_PATCH_COMPONENTS: types::GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_LOCATIONS: types::GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_VECTORS: types::GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTERS: types::GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_BINDINGS: types::GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_STRIDE: types::GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_IMAGE_UNIFORMS: types::GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_VECTORS: types::GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_FLOAT: types::GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_INT: types::GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)] pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)] pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_SAMPLE_SHADING_VALUE: types::GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const MULTIPLY: types::GLenum = 0x9294;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_LINE_WIDTH_GRANULARITY: types::GLenum = 0x9382;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_LINE_WIDTH_RANGE: types::GLenum = 0x9381;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_LENGTH: types::GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION: types::GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_ACTIVE_VARIABLES: types::GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SAMPLE_COUNTS: types::GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADER_BINARY_FORMATS: types::GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)] pub const OFFSET: types::GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const OVERLAY: types::GLenum = 0x9296;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PATCHES: types::GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_VERTICES: types::GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_BUFFER_BARRIER_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_BOUNDING_BOX: types::GLenum = 0x92BE;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FIXED_INDEX: types::GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: types::GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_LENGTH: types::GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_RETRIEVABLE_HINT: types::GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_INPUT: types::GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_OUTPUT: types::GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE_BINDING: types::GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_SEPARABLE: types::GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)] pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)] pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)] pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)] pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)] pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)] pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)] pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)] pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)] pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)] pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)] pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)] pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)] pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BITS: types::GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)] pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESET_NOTIFICATION_STRATEGY: types::GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)] pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)] pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)] pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)] pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)] pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)] pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)] pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)] pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)] pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)] pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)] pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)] pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)] pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: types::GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_SHADING: types::GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SCREEN: types::GLenum = 0x9295;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_BINARY_FORMATS: types::GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_COMPILER: types::GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BARRIER_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BLOCK: types::GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER: types::GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_BINDING: types::GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_SIZE: types::GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_START: types::GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)] pub const SOFTLIGHT: types::GLenum = 0x929C;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BITS: types::GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: types::GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_OUTPUT_VERTICES: types::GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER: types::GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER: types::GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_MODE: types::GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_POINT_MODE: types::GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_SPACING: types::GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_VERTEX_ORDER: types::GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: types::GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_BINDING: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET: types::GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_SIZE: types::GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FETCH_BARRIER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_FORMAT: types::GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_LEVELS: types::GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_UPDATE_BARRIER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_SIZE: types::GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_STRIDE: types::GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK: types::GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BARRIER_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BINDING: types::GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING: types::GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const TYPE: types::GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNDEFINED_VERTEX: types::GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM: types::GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BARRIER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK: types::GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)] pub const UNKNOWN_CONTEXT_RESET: types::GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_ATOMIC_COUNTER: types::GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D: types::GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_ARRAY: types::GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_3D: types::GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_BUFFER: types::GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE: types::GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_BINDING: types::GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_BUFFER: types::GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_DIVISOR: types::GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_OFFSET: types::GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_STRIDE: types::GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ActiveShaderProgram(pipeline: types::GLuint, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::ActiveShaderProgram.f)(pipeline, program) }
/// Fallbacks: ActiveTextureARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ActiveTexture(texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ActiveTexture.f)(texture) }
/// Fallbacks: AttachObjectARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn AttachShader(program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::AttachShader.f)(program, shader) }
/// Fallbacks: BeginQueryARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BeginQuery(target: types::GLenum, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BeginQuery.f)(target, id) }
/// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BeginTransformFeedback(primitiveMode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::BeginTransformFeedback.f)(primitiveMode) }
/// Fallbacks: BindAttribLocationARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindAttribLocation(program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(storage::BindAttribLocation.f)(program, index, name) }
/// Fallbacks: BindBufferARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindBuffer(target: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindBuffer.f)(target, buffer) }
/// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindBufferBase(target: types::GLenum, index: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(storage::BindBufferBase.f)(target, index, buffer) }
/// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindBufferRange(target: types::GLenum, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::BindBufferRange.f)(target, index, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindFramebuffer(target: types::GLenum, framebuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindFramebuffer.f)(target, framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindImageTexture(unit: types::GLuint, texture: types::GLuint, level: types::GLint, layered: types::GLboolean, layer: types::GLint, access: types::GLenum, format: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLboolean, types::GLint, types::GLenum, types::GLenum) -> ()>(storage::BindImageTexture.f)(unit, texture, level, layered, layer, access, format) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindProgramPipeline(pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::BindProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindRenderbuffer(target: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindRenderbuffer.f)(target, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindSampler(unit: types::GLuint, sampler: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::BindSampler.f)(unit, sampler) }
/// Fallbacks: BindTextureEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindTexture(target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindTransformFeedback(target: types::GLenum, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTransformFeedback.f)(target, id) }
/// Fallbacks: BindVertexArrayOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindVertexArray(array: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::BindVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindVertexBuffer(bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(storage::BindVertexBuffer.f)(bindingindex, buffer, offset, stride) }
/// Fallbacks: BlendBarrierKHR, BlendBarrierNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendBarrier() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::BlendBarrier.f)() }
/// Fallbacks: BlendColorEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::BlendColor.f)(red, green, blue, alpha) }
/// Fallbacks: BlendEquationEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendEquation(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::BlendEquation.f)(mode) }
/// Fallbacks: BlendEquationSeparateEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendEquationSeparate(modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendEquationSeparate.f)(modeRGB, modeAlpha) }
/// Fallbacks: BlendEquationSeparateIndexedAMD, BlendEquationSeparateiARB, BlendEquationSeparateiEXT, BlendEquationSeparateiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendEquationSeparatei(buf: types::GLuint, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(storage::BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) }
/// Fallbacks: BlendEquationIndexedAMD, BlendEquationiARB, BlendEquationiEXT, BlendEquationiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendEquationi(buf: types::GLuint, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::BlendEquationi.f)(buf, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendFunc.f)(sfactor, dfactor) }
/// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFuncSeparate(sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
/// Fallbacks: BlendFuncSeparateIndexedAMD, BlendFuncSeparateiARB, BlendFuncSeparateiEXT, BlendFuncSeparateiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFuncSeparatei(buf: types::GLuint, srcRGB: types::GLenum, dstRGB: types::GLenum, srcAlpha: types::GLenum, dstAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
/// Fallbacks: BlendFuncIndexedAMD, BlendFunciARB, BlendFunciEXT, BlendFunciOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFunci(buf: types::GLuint, src: types::GLenum, dst: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(storage::BlendFunci.f)(buf, src, dst) }
/// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlitFramebuffer(srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLbitfield, types::GLenum) -> ()>(storage::BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
/// Fallbacks: BufferDataARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BufferData(target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLenum) -> ()>(storage::BufferData.f)(target, size, data, usage) }
/// Fallbacks: BufferSubDataARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(storage::BufferSubData.f)(target, offset, size, data) }
/// Fallbacks: CheckFramebufferStatusEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CheckFramebufferStatus(target: types::GLenum) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(storage::CheckFramebufferStatus.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Clear(mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearBufferfi(buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLfloat, types::GLint) -> ()>(storage::ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearBufferfv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLfloat) -> ()>(storage::ClearBufferfv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearBufferiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLint) -> ()>(storage::ClearBufferiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearBufferuiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLuint) -> ()>(storage::ClearBufferuiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ClearColor.f)(red, green, blue, alpha) }
/// Fallbacks: ClearDepthfOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearDepthf(d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::ClearDepthf.f)(d) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearStencil(s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::ClearStencil.f)(s) }
/// Fallbacks: ClientWaitSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClientWaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> types::GLenum>(storage::ClientWaitSync.f)(sync, flags, timeout) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMask.f)(red, green, blue, alpha) }
/// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorMaski(index: types::GLuint, r: types::GLboolean, g: types::GLboolean, b: types::GLboolean, a: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMaski.f)(index, r, g, b, a) }
/// Fallbacks: CompileShaderARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompileShader(shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::CompileShader.f)(shader) }
/// Fallbacks: CompressedTexImage2DARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
/// Fallbacks: CompressedTexImage3DARB, CompressedTexImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
/// Fallbacks: CompressedTexSubImage2DARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
/// Fallbacks: CompressedTexSubImage3DARB, CompressedTexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
/// Fallbacks: CopyBufferSubDataNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyBufferSubData(readTarget: types::GLenum, writeTarget: types::GLenum, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(storage::CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) }
/// Fallbacks: CopyImageSubDataEXT, CopyImageSubDataOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyImageSubData(srcName: types::GLuint, srcTarget: types::GLenum, srcLevel: types::GLint, srcX: types::GLint, srcY: types::GLint, srcZ: types::GLint, dstName: types::GLuint, dstTarget: types::GLenum, dstLevel: types::GLint, dstX: types::GLint, dstY: types::GLint, dstZ: types::GLint, srcWidth: types::GLsizei, srcHeight: types::GLsizei, srcDepth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
/// Fallbacks: CopyTexImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
/// Fallbacks: CopyTexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
/// Fallbacks: CopyTexSubImage3DEXT, CopyTexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
/// Fallbacks: CreateProgramObjectARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CreateProgram() -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLuint>(storage::CreateProgram.f)() }
/// Fallbacks: CreateShaderObjectARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CreateShader(type_: types::GLenum) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLuint>(storage::CreateShader.f)(type_) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CreateShaderProgramv(type_: types::GLenum, count: types::GLsizei, strings: *const *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const *const types::GLchar) -> types::GLuint>(storage::CreateShaderProgramv.f)(type_, count, strings) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CullFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::CullFace.f)(mode) }
/// Fallbacks: DebugMessageCallbackARB, DebugMessageCallbackKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DebugMessageCallback(callback: types::GLDEBUGPROC, userParam: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLDEBUGPROC, *const __gl_imports::raw::c_void) -> ()>(storage::DebugMessageCallback.f)(callback, userParam) }
/// Fallbacks: DebugMessageControlARB, DebugMessageControlKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DebugMessageControl(source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *const types::GLuint, types::GLboolean) -> ()>(storage::DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
/// Fallbacks: DebugMessageInsertARB, DebugMessageInsertKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DebugMessageInsert(source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLenum, types::GLsizei, *const types::GLchar) -> ()>(storage::DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
/// Fallbacks: DeleteBuffersARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteBuffers(n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteBuffers.f)(n, buffers) }
/// Fallbacks: DeleteFramebuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteFramebuffers(n: types::GLsizei, framebuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteProgram(program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DeleteProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteProgramPipelines(n: types::GLsizei, pipelines: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteProgramPipelines.f)(n, pipelines) }
/// Fallbacks: DeleteQueriesARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteQueries(n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteQueries.f)(n, ids) }
/// Fallbacks: DeleteRenderbuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteRenderbuffers(n: types::GLsizei, renderbuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteSamplers(count: types::GLsizei, samplers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteShader(shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DeleteShader.f)(shader) }
/// Fallbacks: DeleteSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteSync(sync: types::GLsync) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> ()>(storage::DeleteSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTextures.f)(n, textures) }
/// Fallbacks: DeleteTransformFeedbacksNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteTransformFeedbacks(n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTransformFeedbacks.f)(n, ids) }
/// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteVertexArrays(n: types::GLsizei, arrays: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthFunc(func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthMask(flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(storage::DepthMask.f)(flag) }
/// Fallbacks: DepthRangefOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthRangef(n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::DepthRangef.f)(n, f) }
/// Fallbacks: DetachObjectARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DetachShader(program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::DetachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Disable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Disable.f)(cap) }
/// Fallbacks: DisableVertexAttribArrayARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DisableVertexAttribArray(index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DisableVertexAttribArray.f)(index) }
/// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Disablei(target: types::GLenum, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::Disablei.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DispatchCompute(num_groups_x: types::GLuint, num_groups_y: types::GLuint, num_groups_z: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DispatchComputeIndirect(indirect: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLintptr) -> ()>(storage::DispatchComputeIndirect.f)(indirect) }
/// Fallbacks: DrawArraysEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(storage::DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawArraysIndirect(mode: types::GLenum, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawArraysIndirect.f)(mode, indirect) }
/// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawArraysInstanced(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::DrawArraysInstanced.f)(mode, first, count, instancecount) }
/// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawBuffers(n: types::GLsizei, bufs: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLenum) -> ()>(storage::DrawBuffers.f)(n, bufs) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawElements.f)(mode, count, type_, indices) }
/// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElementsBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLint) -> ()>(storage::DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElementsIndirect(mode: types::GLenum, type_: types::GLenum, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawElementsIndirect.f)(mode, type_, indirect) }
/// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElementsInstanced(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(storage::DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
/// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElementsInstancedBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei, types::GLint) -> ()>(storage::DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
/// Fallbacks: DrawRangeElementsEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawRangeElements(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawRangeElements.f)(mode, start, end, count, type_, indices) }
/// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawRangeElementsBaseVertex(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void, types::GLint) -> ()>(storage::DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Enable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Enable.f)(cap) }
/// Fallbacks: EnableVertexAttribArrayARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EnableVertexAttribArray(index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::EnableVertexAttribArray.f)(index) }
/// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Enablei(target: types::GLenum, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::Enablei.f)(target, index) }
/// Fallbacks: EndQueryARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EndQuery(target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::EndQuery.f)(target) }
/// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EndTransformFeedback() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.f)() }
/// Fallbacks: FenceSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FenceSync(condition: types::GLenum, flags: types::GLbitfield) -> types::GLsync { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLbitfield) -> types::GLsync>(storage::FenceSync.f)(condition, flags) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Finish() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Flush() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.f)() }
/// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FlushMappedBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr) -> ()>(storage::FlushMappedBufferRange.f)(target, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::FramebufferParameteri.f)(target, pname, param) }
/// Fallbacks: FramebufferRenderbufferEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferRenderbuffer(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
/// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferTexture(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture.f)(target, attachment, texture, level) }
/// Fallbacks: FramebufferTexture2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferTexture2D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
/// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferTextureLayer(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(storage::FramebufferTextureLayer.f)(target, attachment, texture, level, layer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FrontFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::FrontFace.f)(mode) }
/// Fallbacks: GenBuffersARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenBuffers(n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenBuffers.f)(n, buffers) }
/// Fallbacks: GenFramebuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenProgramPipelines(n: types::GLsizei, pipelines: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenProgramPipelines.f)(n, pipelines) }
/// Fallbacks: GenQueriesARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenQueries(n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenQueries.f)(n, ids) }
/// Fallbacks: GenRenderbuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenSamplers(count: types::GLsizei, samplers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTextures.f)(n, textures) }
/// Fallbacks: GenTransformFeedbacksNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenTransformFeedbacks(n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTransformFeedbacks.f)(n, ids) }
/// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenVertexArrays.f)(n, arrays) }
/// Fallbacks: GenerateMipmapEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenerateMipmap(target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::GenerateMipmap.f)(target) }
/// Fallbacks: GetActiveAttribARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetActiveAttrib(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
/// Fallbacks: GetActiveUniformARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetActiveUniform(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetActiveUniformBlockName(program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetActiveUniformBlockiv(program: types::GLuint, uniformBlockIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetActiveUniformsiv(program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetAttachedShaders(program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLuint) -> ()>(storage::GetAttachedShaders.f)(program, maxCount, count, shaders) }
/// Fallbacks: GetAttribLocationARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetAttribLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetAttribLocation.f)(program, name) }
/// Fallbacks: GetBooleanIndexedvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBooleani_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLboolean) -> ()>(storage::GetBooleani_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(storage::GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBufferParameteri64v(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint64) -> ()>(storage::GetBufferParameteri64v.f)(target, pname, params) }
/// Fallbacks: GetBufferParameterivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetBufferParameteriv.f)(target, pname, params) }
/// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBufferPointerv(target: types::GLenum, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetBufferPointerv.f)(target, pname, params) }
/// Fallbacks: GetDebugMessageLogARB, GetDebugMessageLogKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetDebugMessageLog(count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLenum, *mut types::GLenum, *mut types::GLuint, *mut types::GLenum, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(storage::GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetError() -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetFloatv.f)(pname, data) }
/// Fallbacks: GetFragDataLocationEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFragDataLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetFragDataLocation.f)(program, name) }
/// Fallbacks: GetFramebufferAttachmentParameterivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFramebufferAttachmentParameteriv(target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFramebufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetFramebufferParameteriv.f)(target, pname, params) }
/// Fallbacks: GetGraphicsResetStatusKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetGraphicsResetStatus() -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetGraphicsResetStatus.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetInteger64i_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint64) -> ()>(storage::GetInteger64i_v.f)(target, index, data) }
/// Fallbacks: GetInteger64vAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetInteger64v(pname: types::GLenum, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint64) -> ()>(storage::GetInteger64v.f)(pname, data) }
/// Fallbacks: GetIntegerIndexedvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetIntegeri_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint) -> ()>(storage::GetIntegeri_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(storage::GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetInternalformativ(target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint) -> ()>(storage::GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
/// Fallbacks: GetMultisamplefvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMultisamplefv(pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(storage::GetMultisamplefv.f)(pname, index, val) }
/// Fallbacks: GetObjectLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetObjectLabel(identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetObjectLabel.f)(identifier, name, bufSize, length, label) }
/// Fallbacks: GetObjectPtrLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetObjectPtrLabel(ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
/// Fallbacks: GetPointervEXT, GetPointervKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPointerv(pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetPointerv.f)(pname, params) }
/// Fallbacks: GetProgramBinaryOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramBinary(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, binaryFormat: *mut types::GLenum, binary: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(storage::GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramInfoLog(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramInterfaceiv(program: types::GLuint, programInterface: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramPipelineInfoLog(pipeline: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramPipelineiv(pipeline: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramPipelineiv.f)(pipeline, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramResourceIndex(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLuint>(storage::GetProgramResourceIndex.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramResourceLocation(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(storage::GetProgramResourceLocation.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramResourceName(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramResourceiv(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, propCount: types::GLsizei, props: *const types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *const types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(storage::GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetProgramiv(program: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramiv.f)(program, pname, params) }
/// Fallbacks: GetQueryObjectuivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetQueryObjectuiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetQueryObjectuiv.f)(id, pname, params) }
/// Fallbacks: GetQueryivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetQueryiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetQueryiv.f)(target, pname, params) }
/// Fallbacks: GetRenderbufferParameterivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetRenderbufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetRenderbufferParameteriv.f)(target, pname, params) }
/// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetSamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetSamplerParameterIiv.f)(sampler, pname, params) }
/// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetSamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetSamplerParameterIuiv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetSamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetSamplerParameterfv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetSamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetSamplerParameteriv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetShaderInfoLog(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetShaderPrecisionFormat(shadertype: types::GLenum, precisiontype: types::GLenum, range: *mut types::GLint, precision: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint, *mut types::GLint) -> ()>(storage::GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
/// Fallbacks: GetShaderSourceARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetShaderSource(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetShaderSource.f)(shader, bufSize, length, source) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetShaderiv(shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetShaderiv.f)(shader, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetString(name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(storage::GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetStringi(name: types::GLenum, index: types::GLuint) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> *const types::GLubyte>(storage::GetStringi.f)(name, index) }
/// Fallbacks: GetSyncivAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetSynciv(sync: types::GLsync, pname: types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(storage::GetSynciv.f)(sync, pname, bufSize, length, values) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
/// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameterIiv.f)(target, pname, params) }
/// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLuint) -> ()>(storage::GetTexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameteriv.f)(target, pname, params) }
/// Fallbacks: GetTransformFeedbackVaryingEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTransformFeedbackVarying(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformBlockIndex(program: types::GLuint, uniformBlockName: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLuint>(storage::GetUniformBlockIndex.f)(program, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformIndices(program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *mut types::GLuint) -> ()>(storage::GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
/// Fallbacks: GetUniformLocationARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetUniformLocation.f)(program, name) }
/// Fallbacks: GetUniformfvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformfv(program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> ()>(storage::GetUniformfv.f)(program, location, params) }
/// Fallbacks: GetUniformivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformiv(program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> ()>(storage::GetUniformiv.f)(program, location, params) }
/// Fallbacks: GetUniformuivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetUniformuiv(program: types::GLuint, location: types::GLint, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLuint) -> ()>(storage::GetUniformuiv.f)(program, location, params) }
/// Fallbacks: GetVertexAttribIivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetVertexAttribIiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexAttribIiv.f)(index, pname, params) }
/// Fallbacks: GetVertexAttribIuivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetVertexAttribIuiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetVertexAttribIuiv.f)(index, pname, params) }
/// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetVertexAttribPointerv(index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetVertexAttribPointerv.f)(index, pname, pointer) }
/// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetVertexAttribfv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetVertexAttribfv.f)(index, pname, params) }
/// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetVertexAttribiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexAttribiv.f)(index, pname, params) }
/// Fallbacks: GetnUniformfvKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetnUniformfv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLfloat) -> ()>(storage::GetnUniformfv.f)(program, location, bufSize, params) }
/// Fallbacks: GetnUniformivKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetnUniformiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLint) -> ()>(storage::GetnUniformiv.f)(program, location, bufSize, params) }
/// Fallbacks: GetnUniformuivKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetnUniformuiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLuint) -> ()>(storage::GetnUniformuiv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Hint(target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn InvalidateFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum) -> ()>(storage::InvalidateFramebuffer.f)(target, numAttachments, attachments) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn InvalidateSubFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
/// Fallbacks: IsBufferARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsBuffer(buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsEnabled(cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::IsEnabled.f)(cap) }
/// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsEnabledi(target: types::GLenum, index: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> types::GLboolean>(storage::IsEnabledi.f)(target, index) }
/// Fallbacks: IsFramebufferEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsFramebuffer(framebuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsFramebuffer.f)(framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsProgram(program: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsProgramPipeline(pipeline: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsProgramPipeline.f)(pipeline) }
/// Fallbacks: IsQueryARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsQuery(id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsQuery.f)(id) }
/// Fallbacks: IsRenderbufferEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsRenderbuffer(renderbuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsRenderbuffer.f)(renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsSampler(sampler: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsSampler.f)(sampler) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsShader(shader: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsShader.f)(shader) }
/// Fallbacks: IsSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsSync(sync: types::GLsync) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> types::GLboolean>(storage::IsSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsTexture(texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTexture.f)(texture) }
/// Fallbacks: IsTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsTransformFeedback(id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTransformFeedback.f)(id) }
/// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsVertexArray(array: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LineWidth(width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::LineWidth.f)(width) }
/// Fallbacks: LinkProgramARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LinkProgram(program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::LinkProgram.f)(program) }
/// Fallbacks: MapBufferRangeEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLbitfield) -> *mut __gl_imports::raw::c_void>(storage::MapBufferRange.f)(target, offset, length, access) }
/// Fallbacks: MemoryBarrierEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MemoryBarrier(barriers: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::MemoryBarrier.f)(barriers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MemoryBarrierByRegion(barriers: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::MemoryBarrierByRegion.f)(barriers) }
/// Fallbacks: MinSampleShadingARB, MinSampleShadingOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MinSampleShading(value: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::MinSampleShading.f)(value) }
/// Fallbacks: ObjectLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ObjectLabel(identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(storage::ObjectLabel.f)(identifier, name, length, label) }
/// Fallbacks: ObjectPtrLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ObjectPtrLabel(ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *const types::GLchar) -> ()>(storage::ObjectPtrLabel.f)(ptr, length, label) }
/// Fallbacks: PatchParameteriEXT, PatchParameteriOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PatchParameteri(pname: types::GLenum, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PatchParameteri.f)(pname, value) }
/// Fallbacks: PauseTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PauseTransformFeedback() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PauseTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelStorei(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::PolygonOffset.f)(factor, units) }
/// Fallbacks: PopDebugGroupKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopDebugGroup() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroup.f)() }
/// Fallbacks: PrimitiveBoundingBoxARB, PrimitiveBoundingBoxEXT, PrimitiveBoundingBoxOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PrimitiveBoundingBox(minX: types::GLfloat, minY: types::GLfloat, minZ: types::GLfloat, minW: types::GLfloat, maxX: types::GLfloat, maxY: types::GLfloat, maxZ: types::GLfloat, maxW: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::PrimitiveBoundingBox.f)(minX, minY, minZ, minW, maxX, maxY, maxZ, maxW) }
/// Fallbacks: ProgramBinaryOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramBinary(program: types::GLuint, binaryFormat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(storage::ProgramBinary.f)(program, binaryFormat, binary, length) }
/// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramParameteri(program: types::GLuint, pname: types::GLenum, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::ProgramParameteri.f)(program, pname, value) }
/// Fallbacks: ProgramUniform1fEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1f(program: types::GLuint, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat) -> ()>(storage::ProgramUniform1f.f)(program, location, v0) }
/// Fallbacks: ProgramUniform1fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform1fv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform1iEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1i(program: types::GLuint, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform1i.f)(program, location, v0) }
/// Fallbacks: ProgramUniform1ivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform1iv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform1uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1ui(program: types::GLuint, location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint) -> ()>(storage::ProgramUniform1ui.f)(program, location, v0) }
/// Fallbacks: ProgramUniform1uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform1uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform1uiv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform2fEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform2f.f)(program, location, v0, v1) }
/// Fallbacks: ProgramUniform2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform2fv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform2iEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform2i.f)(program, location, v0, v1) }
/// Fallbacks: ProgramUniform2ivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform2iv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform2uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform2ui.f)(program, location, v0, v1) }
/// Fallbacks: ProgramUniform2uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform2uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform2uiv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform3fEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform3f.f)(program, location, v0, v1, v2) }
/// Fallbacks: ProgramUniform3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform3fv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform3iEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform3i.f)(program, location, v0, v1, v2) }
/// Fallbacks: ProgramUniform3ivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform3iv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform3uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform3ui.f)(program, location, v0, v1, v2) }
/// Fallbacks: ProgramUniform3uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform3uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform3uiv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform4fEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform4f.f)(program, location, v0, v1, v2, v3) }
/// Fallbacks: ProgramUniform4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform4fv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform4iEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform4i.f)(program, location, v0, v1, v2, v3) }
/// Fallbacks: ProgramUniform4ivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform4iv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniform4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) }
/// Fallbacks: ProgramUniform4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniform4uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform4uiv.f)(program, location, count, value) }
/// Fallbacks: ProgramUniformMatrix2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix2x3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix2x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix2x4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix2x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix3x2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix3x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix3x4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix3x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix4x2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix4x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
/// Fallbacks: ProgramUniformMatrix4x3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ProgramUniformMatrix4x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
/// Fallbacks: PushDebugGroupKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushDebugGroup(source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(storage::PushDebugGroup.f)(source, id, length, message) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadBuffer(src: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ReadBuffer.f)(src) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
/// Fallbacks: ReadnPixelsARB, ReadnPixelsEXT, ReadnPixelsKHR
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadnPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(storage::ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReleaseShaderCompiler() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.f)() }
/// Fallbacks: RenderbufferStorageEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RenderbufferStorage(target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::RenderbufferStorage.f)(target, internalformat, width, height) }
/// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RenderbufferStorageMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) }
/// Fallbacks: ResumeTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ResumeTransformFeedback() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::ResumeTransformFeedback.f)() }
/// Fallbacks: SampleCoverageARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SampleCoverage(value: types::GLfloat, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(storage::SampleCoverage.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SampleMaski(maskNumber: types::GLuint, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield) -> ()>(storage::SampleMaski.f)(maskNumber, mask) }
/// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::SamplerParameterIiv.f)(sampler, pname, param) }
/// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> ()>(storage::SamplerParameterIuiv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameterf(sampler: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> ()>(storage::SamplerParameterf.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> ()>(storage::SamplerParameterfv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameteri(sampler: types::GLuint, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::SamplerParameteri.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::SamplerParameteriv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ShaderBinary(count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(storage::ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
/// Fallbacks: ShaderSourceARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *const types::GLint) -> ()>(storage::ShaderSource.f)(shader, count, string, length) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilFuncSeparate(face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFuncSeparate.f)(face, func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilMask(mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilMaskSeparate(face: types::GLenum, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::StencilMaskSeparate.f)(face, mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOp.f)(fail, zfail, zpass) }
/// Fallbacks: StencilOpSeparateATI
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilOpSeparate(face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
/// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexBuffer(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::TexBuffer.f)(target, internalformat, buffer) }
/// Fallbacks: TexBufferRangeEXT, TexBufferRangeOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexBufferRange(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::TexBufferRange.f)(target, internalformat, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
/// Fallbacks: TexImage3DEXT, TexImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
/// Fallbacks: TexParameterIivEXT, TexParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameterIiv.f)(target, pname, params) }
/// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::TexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameteriv.f)(target, pname, params) }
/// Fallbacks: TexStorage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexStorage2D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::TexStorage2D.f)(target, levels, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexStorage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
/// Fallbacks: TexStorage3DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexStorage3D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::TexStorage3D.f)(target, levels, internalformat, width, height, depth) }
/// Fallbacks: TexStorage3DMultisampleOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexStorage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
/// Fallbacks: TexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
/// Fallbacks: TexSubImage3DEXT, TexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
/// Fallbacks: TransformFeedbackVaryingsEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TransformFeedbackVaryings(program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, types::GLenum) -> ()>(storage::TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
/// Fallbacks: Uniform1fARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1f(location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(storage::Uniform1f.f)(location, v0) }
/// Fallbacks: Uniform1fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform1fv.f)(location, count, value) }
/// Fallbacks: Uniform1iARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1i(location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::Uniform1i.f)(location, v0) }
/// Fallbacks: Uniform1ivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform1iv.f)(location, count, value) }
/// Fallbacks: Uniform1uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1ui(location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint) -> ()>(storage::Uniform1ui.f)(location, v0) }
/// Fallbacks: Uniform1uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform1uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform1uiv.f)(location, count, value) }
/// Fallbacks: Uniform2fARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform2f.f)(location, v0, v1) }
/// Fallbacks: Uniform2fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform2fv.f)(location, count, value) }
/// Fallbacks: Uniform2iARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2i(location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform2i.f)(location, v0, v1) }
/// Fallbacks: Uniform2ivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform2iv.f)(location, count, value) }
/// Fallbacks: Uniform2uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint) -> ()>(storage::Uniform2ui.f)(location, v0, v1) }
/// Fallbacks: Uniform2uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform2uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform2uiv.f)(location, count, value) }
/// Fallbacks: Uniform3fARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform3f.f)(location, v0, v1, v2) }
/// Fallbacks: Uniform3fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform3fv.f)(location, count, value) }
/// Fallbacks: Uniform3iARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform3i.f)(location, v0, v1, v2) }
/// Fallbacks: Uniform3ivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform3iv.f)(location, count, value) }
/// Fallbacks: Uniform3uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Uniform3ui.f)(location, v0, v1, v2) }
/// Fallbacks: Uniform3uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform3uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform3uiv.f)(location, count, value) }
/// Fallbacks: Uniform4fARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform4f.f)(location, v0, v1, v2, v3) }
/// Fallbacks: Uniform4fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform4fv.f)(location, count, value) }
/// Fallbacks: Uniform4iARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform4i.f)(location, v0, v1, v2, v3) }
/// Fallbacks: Uniform4ivARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform4iv.f)(location, count, value) }
/// Fallbacks: Uniform4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Uniform4ui.f)(location, v0, v1, v2, v3) }
/// Fallbacks: Uniform4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Uniform4uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform4uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformBlockBinding(program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) }
/// Fallbacks: UniformMatrix2fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix2x3fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix2x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2x3fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix2x4fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix2x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2x4fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix3fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix3x2fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix3x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3x2fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix3x4fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix3x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3x4fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix4fvARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix4x2fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix4x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4x2fv.f)(location, count, transpose, value) }
/// Fallbacks: UniformMatrix4x3fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UniformMatrix4x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4x3fv.f)(location, count, transpose, value) }
/// Fallbacks: UnmapBufferARB, UnmapBufferOES
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UnmapBuffer(target: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::UnmapBuffer.f)(target) }
/// Fallbacks: UseProgramObjectARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UseProgram(program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::UseProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UseProgramStages(pipeline: types::GLuint, stages: types::GLbitfield, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield, types::GLuint) -> ()>(storage::UseProgramStages.f)(pipeline, stages, program) }
/// Fallbacks: ValidateProgramARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ValidateProgram(program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::ValidateProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ValidateProgramPipeline(pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::ValidateProgramPipeline.f)(pipeline) }
/// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib1f(index: types::GLuint, x: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(storage::VertexAttrib1f.f)(index, x) }
/// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib1fv(index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib1fv.f)(index, v) }
/// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib2f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib2f.f)(index, x, y) }
/// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib2fv(index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib2fv.f)(index, v) }
/// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib3f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib3f.f)(index, x, y, z) }
/// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib3fv(index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib3fv.f)(index, v) }
/// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib4f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib4f.f)(index, x, y, z, w) }
/// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttrib4fv(index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib4fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribBinding(attribindex: types::GLuint, bindingindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexAttribBinding.f)(attribindex, bindingindex) }
/// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribDivisor(index: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexAttribDivisor.f)(index, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) }
/// Fallbacks: VertexAttribI4iEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribI4i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::VertexAttribI4i.f)(index, x, y, z, w) }
/// Fallbacks: VertexAttribI4ivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribI4iv(index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttribI4iv.f)(index, v) }
/// Fallbacks: VertexAttribI4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribI4ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexAttribI4ui.f)(index, x, y, z, w) }
/// Fallbacks: VertexAttribI4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribI4uiv(index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttribI4uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribIFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(storage::VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) }
/// Fallbacks: VertexAttribIPointerEXT
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribIPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
/// Fallbacks: VertexAttribPointerARB
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexBindingDivisor(bindingindex: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexBindingDivisor.f)(bindingindex, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Viewport.f)(x, y, width, height) }
/// Fallbacks: WaitSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn WaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> ()>(storage::WaitSync.f)(sync, flags, timeout) }

        #[allow(missing_copy_implementations)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr { f: missing_fn_panic as *const __gl_imports::raw::c_void, is_loaded: false }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }
        }
    
mod storage {
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            use super::__gl_imports::raw;
            use super::FnPtr;
pub static mut ActiveShaderProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ActiveTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut AttachShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BeginQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BeginTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindAttribLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindBufferBase: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindImageTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindSampler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindVertexArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindVertexBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendBarrier: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendEquation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendEquationSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendEquationSeparatei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendEquationi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFuncSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFuncSeparatei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFunci: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlitFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CheckFramebufferStatus: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Clear: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearBufferfi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearBufferfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearBufferiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearBufferuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearDepthf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearStencil: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClientWaitSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorMaski: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompileShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyImageSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CreateProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CreateShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CreateShaderProgramv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CullFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DebugMessageCallback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DebugMessageControl: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DebugMessageInsert: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteFramebuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteProgramPipelines: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteQueries: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteRenderbuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteTransformFeedbacks: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteVertexArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthRangef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DetachShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Disable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DisableVertexAttribArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Disablei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DispatchCompute: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DispatchComputeIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawArraysIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawArraysInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElementsBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElementsIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstancedBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawRangeElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawRangeElementsBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Enable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EnableVertexAttribArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Enablei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EndQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EndTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FenceSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Finish: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Flush: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FlushMappedBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferTextureLayer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FrontFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenFramebuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenProgramPipelines: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenQueries: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenRenderbuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenTransformFeedbacks: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenVertexArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenerateMipmap: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetActiveAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetActiveUniform: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformBlockName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformsiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetAttachedShaders: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetAttribLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBooleani_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBooleanv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBufferParameteri64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBufferPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetDebugMessageLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetError: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFloatv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFragDataLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFramebufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetGraphicsResetStatus: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetInteger64i_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetInteger64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetIntegeri_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetIntegerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetInternalformativ: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMultisamplefv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetObjectLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetObjectPtrLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramInterfaceiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramPipelineInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramPipelineiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetProgramiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetQueryObjectuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetQueryiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetShaderInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetShaderSource: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetShaderiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetString: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetStringi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetSynciv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformBlockIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformIndices: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetUniformuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetnUniformfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetnUniformiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetnUniformuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Hint: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut InvalidateFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut InvalidateSubFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsEnabled: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsEnabledi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsSampler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsVertexArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LineWidth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LinkProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MemoryBarrier: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MemoryBarrierByRegion: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MinSampleShading: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ObjectLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ObjectPtrLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PatchParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PauseTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelStorei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonOffset: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopDebugGroup: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PrimitiveBoundingBox: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushDebugGroup: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadnPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReleaseShaderCompiler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RenderbufferStorage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ResumeTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SampleCoverage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SampleMaski: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SamplerParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scissor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ShaderBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ShaderSource: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilFuncSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilMaskSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilOpSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexStorage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexStorage2DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexStorage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexStorage3DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TransformFeedbackVaryings: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Uniform4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformBlockBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UnmapBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UseProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UseProgramStages: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ValidateProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ValidateProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribDivisor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribIFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribIPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexAttribPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexBindingDivisor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Viewport: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut WaitSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
}

            #[allow(non_snake_case)]
            pub mod ActiveShaderProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ActiveShaderProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ActiveShaderProgram = FnPtr::new(metaloadfn(&mut loadfn, "glActiveShaderProgram", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ActiveTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ActiveTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ActiveTexture = FnPtr::new(metaloadfn(&mut loadfn, "glActiveTexture", &["glActiveTextureARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod AttachShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::AttachShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::AttachShader = FnPtr::new(metaloadfn(&mut loadfn, "glAttachShader", &["glAttachObjectARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BeginQuery {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BeginQuery.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BeginQuery = FnPtr::new(metaloadfn(&mut loadfn, "glBeginQuery", &["glBeginQueryARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BeginTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BeginTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BeginTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glBeginTransformFeedback", &["glBeginTransformFeedbackEXT", "glBeginTransformFeedbackNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindAttribLocation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindAttribLocation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, "glBindAttribLocation", &["glBindAttribLocationARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindBuffer", &["glBindBufferARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindBufferBase {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindBufferBase.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindBufferBase = FnPtr::new(metaloadfn(&mut loadfn, "glBindBufferBase", &["glBindBufferBaseEXT", "glBindBufferBaseNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindBufferRange {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindBufferRange.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glBindBufferRange", &["glBindBufferRangeEXT", "glBindBufferRangeNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindFramebuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindFramebuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindFramebuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindImageTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindImageTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindImageTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindImageTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindProgramPipeline {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindProgramPipeline.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glBindProgramPipeline", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindRenderbuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindRenderbuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindRenderbuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindSampler {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindSampler.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindSampler = FnPtr::new(metaloadfn(&mut loadfn, "glBindSampler", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindTexture", &["glBindTextureEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glBindTransformFeedback", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindVertexArray {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindVertexArray.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindVertexArray = FnPtr::new(metaloadfn(&mut loadfn, "glBindVertexArray", &["glBindVertexArrayOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindVertexBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindVertexBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BindVertexBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindVertexBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendBarrier {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendBarrier.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendBarrier = FnPtr::new(metaloadfn(&mut loadfn, "glBlendBarrier", &["glBlendBarrierKHR", "glBlendBarrierNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendColor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendColor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendColor = FnPtr::new(metaloadfn(&mut loadfn, "glBlendColor", &["glBlendColorEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendEquation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendEquation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendEquation = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquation", &["glBlendEquationEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendEquationSeparate {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendEquationSeparate.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendEquationSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationSeparate", &["glBlendEquationSeparateEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendEquationSeparatei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendEquationSeparatei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendEquationSeparatei = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationSeparatei", &["glBlendEquationSeparateIndexedAMD", "glBlendEquationSeparateiARB", "glBlendEquationSeparateiEXT", "glBlendEquationSeparateiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendEquationi {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendEquationi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendEquationi = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationi", &["glBlendEquationIndexedAMD", "glBlendEquationiARB", "glBlendEquationiEXT", "glBlendEquationiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFunc = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFuncSeparate {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFuncSeparate.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFuncSeparate", &["glBlendFuncSeparateEXT", "glBlendFuncSeparateINGR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFuncSeparatei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFuncSeparatei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFuncSeparatei = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFuncSeparatei", &["glBlendFuncSeparateIndexedAMD", "glBlendFuncSeparateiARB", "glBlendFuncSeparateiEXT", "glBlendFuncSeparateiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFunci {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFunci.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFunci = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunci", &["glBlendFuncIndexedAMD", "glBlendFunciARB", "glBlendFunciEXT", "glBlendFunciOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlitFramebuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlitFramebuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BlitFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBlitFramebuffer", &["glBlitFramebufferEXT", "glBlitFramebufferNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BufferData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BufferData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BufferData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferData", &["glBufferDataARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BufferSubData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BufferSubData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::BufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferSubData", &["glBufferSubDataARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CheckFramebufferStatus {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CheckFramebufferStatus.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CheckFramebufferStatus = FnPtr::new(metaloadfn(&mut loadfn, "glCheckFramebufferStatus", &["glCheckFramebufferStatusEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Clear {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Clear.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Clear = FnPtr::new(metaloadfn(&mut loadfn, "glClear", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearBufferfi {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearBufferfi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearBufferfi = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferfi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearBufferfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearBufferfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearBufferfv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearBufferiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearBufferiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearBufferiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearBufferuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearBufferuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearBufferuiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferuiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearColor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearColor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearColor = FnPtr::new(metaloadfn(&mut loadfn, "glClearColor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearDepthf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearDepthf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearDepthf = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepthf", &["glClearDepthfOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearStencil {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearStencil.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearStencil = FnPtr::new(metaloadfn(&mut loadfn, "glClearStencil", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClientWaitSync {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClientWaitSync.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ClientWaitSync = FnPtr::new(metaloadfn(&mut loadfn, "glClientWaitSync", &["glClientWaitSyncAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorMask = FnPtr::new(metaloadfn(&mut loadfn, "glColorMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorMaski {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorMaski.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorMaski = FnPtr::new(metaloadfn(&mut loadfn, "glColorMaski", &["glColorMaskIndexedEXT", "glColorMaskiEXT", "glColorMaskiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompileShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompileShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CompileShader = FnPtr::new(metaloadfn(&mut loadfn, "glCompileShader", &["glCompileShaderARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage2D", &["glCompressedTexImage2DARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexImage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexImage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage3D", &["glCompressedTexImage3DARB", "glCompressedTexImage3DOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage2D", &["glCompressedTexSubImage2DARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexSubImage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexSubImage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage3D", &["glCompressedTexSubImage3DARB", "glCompressedTexSubImage3DOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyBufferSubData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyBufferSubData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glCopyBufferSubData", &["glCopyBufferSubDataNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyImageSubData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyImageSubData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyImageSubData = FnPtr::new(metaloadfn(&mut loadfn, "glCopyImageSubData", &["glCopyImageSubDataEXT", "glCopyImageSubDataOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage2D", &["glCopyTexImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexSubImage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexSubImage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage3D", &["glCopyTexSubImage3DEXT", "glCopyTexSubImage3DOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CreateProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CreateProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CreateProgram = FnPtr::new(metaloadfn(&mut loadfn, "glCreateProgram", &["glCreateProgramObjectARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CreateShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CreateShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CreateShader = FnPtr::new(metaloadfn(&mut loadfn, "glCreateShader", &["glCreateShaderObjectARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CreateShaderProgramv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CreateShaderProgramv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CreateShaderProgramv = FnPtr::new(metaloadfn(&mut loadfn, "glCreateShaderProgramv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CullFace {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CullFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::CullFace = FnPtr::new(metaloadfn(&mut loadfn, "glCullFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DebugMessageCallback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DebugMessageCallback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DebugMessageCallback = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageCallback", &["glDebugMessageCallbackARB", "glDebugMessageCallbackKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DebugMessageControl {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DebugMessageControl.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DebugMessageControl = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageControl", &["glDebugMessageControlARB", "glDebugMessageControlKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DebugMessageInsert {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DebugMessageInsert.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DebugMessageInsert = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageInsert", &["glDebugMessageInsertARB", "glDebugMessageInsertKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteBuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteBuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteBuffers", &["glDeleteBuffersARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteFramebuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteFramebuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteFramebuffers", &["glDeleteFramebuffersEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteProgram = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteProgram", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteProgramPipelines {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteProgramPipelines.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteProgramPipelines", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteQueries {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteQueries.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteQueries = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteQueries", &["glDeleteQueriesARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteRenderbuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteRenderbuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteRenderbuffers", &["glDeleteRenderbuffersEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteSamplers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteSamplers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteSamplers", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteShader = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteShader", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteSync {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteSync.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteSync = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteSync", &["glDeleteSyncAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteTextures {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteTextures = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteTransformFeedbacks {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteTransformFeedbacks.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTransformFeedbacks", &["glDeleteTransformFeedbacksNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteVertexArrays {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteVertexArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteVertexArrays", &["glDeleteVertexArraysAPPLE", "glDeleteVertexArraysOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthFunc = FnPtr::new(metaloadfn(&mut loadfn, "glDepthFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthMask = FnPtr::new(metaloadfn(&mut loadfn, "glDepthMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthRangef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthRangef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthRangef = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangef", &["glDepthRangefOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DetachShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DetachShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DetachShader = FnPtr::new(metaloadfn(&mut loadfn, "glDetachShader", &["glDetachObjectARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Disable {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Disable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Disable = FnPtr::new(metaloadfn(&mut loadfn, "glDisable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DisableVertexAttribArray {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DisableVertexAttribArray.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DisableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, "glDisableVertexAttribArray", &["glDisableVertexAttribArrayARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Disablei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Disablei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Disablei = FnPtr::new(metaloadfn(&mut loadfn, "glDisablei", &["glDisableIndexedEXT", "glDisableiEXT", "glDisableiNV", "glDisableiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DispatchCompute {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DispatchCompute.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DispatchCompute = FnPtr::new(metaloadfn(&mut loadfn, "glDispatchCompute", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DispatchComputeIndirect {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DispatchComputeIndirect.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DispatchComputeIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDispatchComputeIndirect", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawArrays {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArrays", &["glDrawArraysEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawArraysIndirect {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawArraysIndirect.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawArraysIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArraysIndirect", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawArraysInstanced {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawArraysInstanced.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawArraysInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArraysInstanced", &["glDrawArraysInstancedANGLE", "glDrawArraysInstancedARB", "glDrawArraysInstancedEXT", "glDrawArraysInstancedNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawBuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawBuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDrawBuffers", &["glDrawBuffersARB", "glDrawBuffersATI", "glDrawBuffersEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElements {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElements.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElements", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElementsBaseVertex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElementsBaseVertex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsBaseVertex", &["glDrawElementsBaseVertexEXT", "glDrawElementsBaseVertexOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElementsIndirect {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElementsIndirect.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElementsIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsIndirect", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElementsInstanced {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElementsInstanced.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElementsInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstanced", &["glDrawElementsInstancedANGLE", "glDrawElementsInstancedARB", "glDrawElementsInstancedEXT", "glDrawElementsInstancedNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElementsInstancedBaseVertex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElementsInstancedBaseVertex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElementsInstancedBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstancedBaseVertex", &["glDrawElementsInstancedBaseVertexEXT", "glDrawElementsInstancedBaseVertexOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawRangeElements {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawRangeElements.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawRangeElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawRangeElements", &["glDrawRangeElementsEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawRangeElementsBaseVertex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawRangeElementsBaseVertex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawRangeElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawRangeElementsBaseVertex", &["glDrawRangeElementsBaseVertexEXT", "glDrawRangeElementsBaseVertexOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Enable {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Enable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Enable = FnPtr::new(metaloadfn(&mut loadfn, "glEnable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EnableVertexAttribArray {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EnableVertexAttribArray.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::EnableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, "glEnableVertexAttribArray", &["glEnableVertexAttribArrayARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Enablei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Enablei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Enablei = FnPtr::new(metaloadfn(&mut loadfn, "glEnablei", &["glEnableIndexedEXT", "glEnableiEXT", "glEnableiNV", "glEnableiOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EndQuery {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EndQuery.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::EndQuery = FnPtr::new(metaloadfn(&mut loadfn, "glEndQuery", &["glEndQueryARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EndTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EndTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::EndTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glEndTransformFeedback", &["glEndTransformFeedbackEXT", "glEndTransformFeedbackNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FenceSync {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FenceSync.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FenceSync = FnPtr::new(metaloadfn(&mut loadfn, "glFenceSync", &["glFenceSyncAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Finish {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Finish.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Finish = FnPtr::new(metaloadfn(&mut loadfn, "glFinish", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Flush {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Flush.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Flush = FnPtr::new(metaloadfn(&mut loadfn, "glFlush", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FlushMappedBufferRange {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FlushMappedBufferRange.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FlushMappedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glFlushMappedBufferRange", &["glFlushMappedBufferRangeAPPLE", "glFlushMappedBufferRangeEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferParameteri", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferRenderbuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferRenderbuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferRenderbuffer", &["glFramebufferRenderbufferEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferTexture = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture", &["glFramebufferTextureARB", "glFramebufferTextureEXT", "glFramebufferTextureOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferTexture2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferTexture2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferTexture2D = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture2D", &["glFramebufferTexture2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferTextureLayer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferTextureLayer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferTextureLayer = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTextureLayer", &["glFramebufferTextureLayerARB", "glFramebufferTextureLayerEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FrontFace {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FrontFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::FrontFace = FnPtr::new(metaloadfn(&mut loadfn, "glFrontFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenBuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenBuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenBuffers", &["glGenBuffersARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenFramebuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenFramebuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenFramebuffers", &["glGenFramebuffersEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenProgramPipelines {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenProgramPipelines.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, "glGenProgramPipelines", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenQueries {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenQueries.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenQueries = FnPtr::new(metaloadfn(&mut loadfn, "glGenQueries", &["glGenQueriesARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenRenderbuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenRenderbuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenRenderbuffers", &["glGenRenderbuffersEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenSamplers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenSamplers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glGenSamplers", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenTextures {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenTextures = FnPtr::new(metaloadfn(&mut loadfn, "glGenTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenTransformFeedbacks {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenTransformFeedbacks.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, "glGenTransformFeedbacks", &["glGenTransformFeedbacksNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenVertexArrays {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenVertexArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, "glGenVertexArrays", &["glGenVertexArraysAPPLE", "glGenVertexArraysOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenerateMipmap {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenerateMipmap.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GenerateMipmap = FnPtr::new(metaloadfn(&mut loadfn, "glGenerateMipmap", &["glGenerateMipmapEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetActiveAttrib {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetActiveAttrib.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetActiveAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveAttrib", &["glGetActiveAttribARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetActiveUniform {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetActiveUniform.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetActiveUniform = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniform", &["glGetActiveUniformARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetActiveUniformBlockName {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetActiveUniformBlockName.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetActiveUniformBlockName = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformBlockName", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetActiveUniformBlockiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetActiveUniformBlockiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetActiveUniformBlockiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformBlockiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetActiveUniformsiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetActiveUniformsiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetActiveUniformsiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformsiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetAttachedShaders {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetAttachedShaders.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetAttachedShaders = FnPtr::new(metaloadfn(&mut loadfn, "glGetAttachedShaders", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetAttribLocation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetAttribLocation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetAttribLocation", &["glGetAttribLocationARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBooleani_v {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBooleani_v.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBooleani_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleani_v", &["glGetBooleanIndexedvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBooleanv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBooleanv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBooleanv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleanv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBufferParameteri64v {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBufferParameteri64v.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBufferParameteri64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferParameteri64v", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBufferParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBufferParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferParameteriv", &["glGetBufferParameterivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBufferPointerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBufferPointerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBufferPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferPointerv", &["glGetBufferPointervARB", "glGetBufferPointervOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetDebugMessageLog {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetDebugMessageLog.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetDebugMessageLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetDebugMessageLog", &["glGetDebugMessageLogARB", "glGetDebugMessageLogKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetError {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetError.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetError = FnPtr::new(metaloadfn(&mut loadfn, "glGetError", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFloatv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFloatv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFloatv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFloatv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFragDataLocation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFragDataLocation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFragDataLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetFragDataLocation", &["glGetFragDataLocationEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFramebufferAttachmentParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFramebufferAttachmentParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFramebufferAttachmentParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFramebufferAttachmentParameteriv", &["glGetFramebufferAttachmentParameterivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFramebufferParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFramebufferParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFramebufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFramebufferParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetGraphicsResetStatus {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetGraphicsResetStatus.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetGraphicsResetStatus = FnPtr::new(metaloadfn(&mut loadfn, "glGetGraphicsResetStatus", &["glGetGraphicsResetStatusKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetInteger64i_v {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetInteger64i_v.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetInteger64i_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetInteger64i_v", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetInteger64v {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetInteger64v.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetInteger64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetInteger64v", &["glGetInteger64vAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetIntegeri_v {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetIntegeri_v.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetIntegeri_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegeri_v", &["glGetIntegerIndexedvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetIntegerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetIntegerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetIntegerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegerv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetInternalformativ {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetInternalformativ.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetInternalformativ = FnPtr::new(metaloadfn(&mut loadfn, "glGetInternalformativ", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMultisamplefv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMultisamplefv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMultisamplefv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMultisamplefv", &["glGetMultisamplefvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetObjectLabel {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetObjectLabel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, "glGetObjectLabel", &["glGetObjectLabelKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetObjectPtrLabel {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetObjectPtrLabel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, "glGetObjectPtrLabel", &["glGetObjectPtrLabelKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPointerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPointerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramBinary {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramBinary.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramBinary", &["glGetProgramBinaryOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramInfoLog {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramInfoLog.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramInfoLog", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramInterfaceiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramInterfaceiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramInterfaceiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramInterfaceiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramPipelineInfoLog {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramPipelineInfoLog.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramPipelineInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramPipelineInfoLog", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramPipelineiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramPipelineiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramPipelineiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramPipelineiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramResourceIndex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramResourceIndex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramResourceIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceIndex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramResourceLocation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramResourceLocation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramResourceLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceLocation", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramResourceName {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramResourceName.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramResourceName = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceName", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramResourceiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramResourceiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramResourceiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetProgramiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetProgramiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetProgramiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetQueryObjectuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetQueryObjectuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetQueryObjectuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryObjectuiv", &["glGetQueryObjectuivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetQueryiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetQueryiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetQueryiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryiv", &["glGetQueryivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetRenderbufferParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetRenderbufferParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetRenderbufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetRenderbufferParameteriv", &["glGetRenderbufferParameterivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetSamplerParameterIiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetSamplerParameterIiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetSamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterIiv", &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetSamplerParameterIuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetSamplerParameterIuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetSamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterIuiv", &["glGetSamplerParameterIuivEXT", "glGetSamplerParameterIuivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetSamplerParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetSamplerParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetSamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetSamplerParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetSamplerParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetSamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetShaderInfoLog {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetShaderInfoLog.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetShaderInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderInfoLog", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetShaderPrecisionFormat {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetShaderPrecisionFormat.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetShaderPrecisionFormat = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderPrecisionFormat", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetShaderSource {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetShaderSource.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetShaderSource = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderSource", &["glGetShaderSourceARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetShaderiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetShaderiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetShaderiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderiv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetString {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetString.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetString = FnPtr::new(metaloadfn(&mut loadfn, "glGetString", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetStringi {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetStringi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetStringi = FnPtr::new(metaloadfn(&mut loadfn, "glGetStringi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetSynciv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetSynciv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetSynciv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSynciv", &["glGetSyncivAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexLevelParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexLevelParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexLevelParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexLevelParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterIiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterIiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterIiv", &["glGetTexParameterIivEXT", "glGetTexParameterIivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterIuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterIuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterIuiv", &["glGetTexParameterIuivEXT", "glGetTexParameterIuivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTransformFeedbackVarying {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTransformFeedbackVarying.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTransformFeedbackVarying = FnPtr::new(metaloadfn(&mut loadfn, "glGetTransformFeedbackVarying", &["glGetTransformFeedbackVaryingEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformBlockIndex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformBlockIndex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformBlockIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformBlockIndex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformIndices {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformIndices.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformIndices = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformIndices", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformLocation {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformLocation.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformLocation", &["glGetUniformLocationARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformfv", &["glGetUniformfvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformiv", &["glGetUniformivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetUniformuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetUniformuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformuiv", &["glGetUniformuivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetVertexAttribIiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetVertexAttribIiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetVertexAttribIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribIiv", &["glGetVertexAttribIivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetVertexAttribIuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetVertexAttribIuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetVertexAttribIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribIuiv", &["glGetVertexAttribIuivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetVertexAttribPointerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetVertexAttribPointerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetVertexAttribPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribPointerv", &["glGetVertexAttribPointervARB", "glGetVertexAttribPointervNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetVertexAttribfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetVertexAttribfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetVertexAttribfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribfv", &["glGetVertexAttribfvARB", "glGetVertexAttribfvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetVertexAttribiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetVertexAttribiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetVertexAttribiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribiv", &["glGetVertexAttribivARB", "glGetVertexAttribivNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetnUniformfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetnUniformfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetnUniformfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformfv", &["glGetnUniformfvKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetnUniformiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetnUniformiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetnUniformiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformiv", &["glGetnUniformivKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetnUniformuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetnUniformuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::GetnUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformuiv", &["glGetnUniformuivKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Hint {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Hint.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Hint = FnPtr::new(metaloadfn(&mut loadfn, "glHint", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod InvalidateFramebuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::InvalidateFramebuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::InvalidateFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateFramebuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod InvalidateSubFramebuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::InvalidateSubFramebuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::InvalidateSubFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateSubFramebuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsBuffer", &["glIsBufferARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsEnabled {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsEnabled.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsEnabled = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabled", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsEnabledi {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsEnabledi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsEnabledi = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabledi", &["glIsEnabledIndexedEXT", "glIsEnablediEXT", "glIsEnablediNV", "glIsEnablediOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsFramebuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsFramebuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsFramebuffer", &["glIsFramebufferEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsProgram = FnPtr::new(metaloadfn(&mut loadfn, "glIsProgram", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsProgramPipeline {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsProgramPipeline.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glIsProgramPipeline", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsQuery {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsQuery.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsQuery = FnPtr::new(metaloadfn(&mut loadfn, "glIsQuery", &["glIsQueryARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsRenderbuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsRenderbuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsRenderbuffer", &["glIsRenderbufferEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsSampler {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsSampler.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsSampler = FnPtr::new(metaloadfn(&mut loadfn, "glIsSampler", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsShader {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsShader.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsShader = FnPtr::new(metaloadfn(&mut loadfn, "glIsShader", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsSync {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsSync.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsSync = FnPtr::new(metaloadfn(&mut loadfn, "glIsSync", &["glIsSyncAPPLE"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsTexture = FnPtr::new(metaloadfn(&mut loadfn, "glIsTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glIsTransformFeedback", &["glIsTransformFeedbackNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsVertexArray {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsVertexArray.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::IsVertexArray = FnPtr::new(metaloadfn(&mut loadfn, "glIsVertexArray", &["glIsVertexArrayAPPLE", "glIsVertexArrayOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LineWidth {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LineWidth.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::LineWidth = FnPtr::new(metaloadfn(&mut loadfn, "glLineWidth", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LinkProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LinkProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::LinkProgram = FnPtr::new(metaloadfn(&mut loadfn, "glLinkProgram", &["glLinkProgramARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapBufferRange {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapBufferRange.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::MapBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glMapBufferRange", &["glMapBufferRangeEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MemoryBarrier {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MemoryBarrier.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::MemoryBarrier = FnPtr::new(metaloadfn(&mut loadfn, "glMemoryBarrier", &["glMemoryBarrierEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MemoryBarrierByRegion {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MemoryBarrierByRegion.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::MemoryBarrierByRegion = FnPtr::new(metaloadfn(&mut loadfn, "glMemoryBarrierByRegion", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MinSampleShading {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MinSampleShading.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::MinSampleShading = FnPtr::new(metaloadfn(&mut loadfn, "glMinSampleShading", &["glMinSampleShadingARB", "glMinSampleShadingOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ObjectLabel {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ObjectLabel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, "glObjectLabel", &["glObjectLabelKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ObjectPtrLabel {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ObjectPtrLabel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, "glObjectPtrLabel", &["glObjectPtrLabelKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PatchParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PatchParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PatchParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glPatchParameteri", &["glPatchParameteriEXT", "glPatchParameteriOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PauseTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PauseTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PauseTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glPauseTransformFeedback", &["glPauseTransformFeedbackNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelStorei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelStorei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelStorei = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStorei", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonOffset {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonOffset.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonOffset = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonOffset", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopDebugGroup {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopDebugGroup.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PopDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, "glPopDebugGroup", &["glPopDebugGroupKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PrimitiveBoundingBox {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PrimitiveBoundingBox.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PrimitiveBoundingBox = FnPtr::new(metaloadfn(&mut loadfn, "glPrimitiveBoundingBox", &["glPrimitiveBoundingBoxARB", "glPrimitiveBoundingBoxEXT", "glPrimitiveBoundingBoxOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramBinary {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramBinary.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, "glProgramBinary", &["glProgramBinaryOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glProgramParameteri", &["glProgramParameteriARB", "glProgramParameteriEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1f", &["glProgramUniform1fEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1fv", &["glProgramUniform1fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1i", &["glProgramUniform1iEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1iv", &["glProgramUniform1ivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1ui", &["glProgramUniform1uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform1uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform1uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1uiv", &["glProgramUniform1uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2f", &["glProgramUniform2fEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2fv", &["glProgramUniform2fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2i", &["glProgramUniform2iEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2iv", &["glProgramUniform2ivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2ui", &["glProgramUniform2uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform2uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform2uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2uiv", &["glProgramUniform2uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3f", &["glProgramUniform3fEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3fv", &["glProgramUniform3fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3i", &["glProgramUniform3iEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3iv", &["glProgramUniform3ivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3ui", &["glProgramUniform3uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform3uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform3uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3uiv", &["glProgramUniform3uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4f", &["glProgramUniform4fEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4fv", &["glProgramUniform4fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4i", &["glProgramUniform4iEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4iv", &["glProgramUniform4ivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4ui", &["glProgramUniform4uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniform4uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniform4uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4uiv", &["glProgramUniform4uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2fv", &["glProgramUniformMatrix2fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix2x3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix2x3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x3fv", &["glProgramUniformMatrix2x3fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix2x4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix2x4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x4fv", &["glProgramUniformMatrix2x4fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3fv", &["glProgramUniformMatrix3fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix3x2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix3x2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x2fv", &["glProgramUniformMatrix3x2fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix3x4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix3x4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x4fv", &["glProgramUniformMatrix3x4fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4fv", &["glProgramUniformMatrix4fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix4x2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix4x2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x2fv", &["glProgramUniformMatrix4x2fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ProgramUniformMatrix4x3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ProgramUniformMatrix4x3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ProgramUniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x3fv", &["glProgramUniformMatrix4x3fvEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushDebugGroup {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushDebugGroup.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::PushDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, "glPushDebugGroup", &["glPushDebugGroupKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glReadBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadPixels {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadPixels", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadnPixels {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadnPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadnPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadnPixels", &["glReadnPixelsARB", "glReadnPixelsEXT", "glReadnPixelsKHR"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReleaseShaderCompiler {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReleaseShaderCompiler.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ReleaseShaderCompiler = FnPtr::new(metaloadfn(&mut loadfn, "glReleaseShaderCompiler", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RenderbufferStorage {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RenderbufferStorage.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::RenderbufferStorage = FnPtr::new(metaloadfn(&mut loadfn, "glRenderbufferStorage", &["glRenderbufferStorageEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RenderbufferStorageMultisample {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RenderbufferStorageMultisample.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::RenderbufferStorageMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glRenderbufferStorageMultisample", &["glRenderbufferStorageMultisampleEXT", "glRenderbufferStorageMultisampleNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ResumeTransformFeedback {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ResumeTransformFeedback.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ResumeTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glResumeTransformFeedback", &["glResumeTransformFeedbackNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SampleCoverage {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SampleCoverage.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SampleCoverage = FnPtr::new(metaloadfn(&mut loadfn, "glSampleCoverage", &["glSampleCoverageARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SampleMaski {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SampleMaski.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SampleMaski = FnPtr::new(metaloadfn(&mut loadfn, "glSampleMaski", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameterIiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameterIiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterIiv", &["glSamplerParameterIivEXT", "glSamplerParameterIivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameterIuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameterIuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterIuiv", &["glSamplerParameterIuivEXT", "glSamplerParameterIuivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameterf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameterf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameteri", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SamplerParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SamplerParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::SamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scissor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scissor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Scissor = FnPtr::new(metaloadfn(&mut loadfn, "glScissor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ShaderBinary {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ShaderBinary.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ShaderBinary = FnPtr::new(metaloadfn(&mut loadfn, "glShaderBinary", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ShaderSource {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ShaderSource.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ShaderSource = FnPtr::new(metaloadfn(&mut loadfn, "glShaderSource", &["glShaderSourceARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilFunc = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilFuncSeparate {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilFuncSeparate.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFuncSeparate", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilMask = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilMaskSeparate {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilMaskSeparate.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilMaskSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMaskSeparate", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilOp {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilOp.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilOp = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOp", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilOpSeparate {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilOpSeparate.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilOpSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOpSeparate", &["glStencilOpSeparateATI"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glTexBuffer", &["glTexBufferARB", "glTexBufferEXT", "glTexBufferOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexBufferRange {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexBufferRange.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glTexBufferRange", &["glTexBufferRangeEXT", "glTexBufferRangeOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexImage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexImage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage3D", &["glTexImage3DEXT", "glTexImage3DOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterIiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterIiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterIiv", &["glTexParameterIivEXT", "glTexParameterIivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterIuiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterIuiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterIuiv", &["glTexParameterIuivEXT", "glTexParameterIuivOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteri", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexStorage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexStorage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexStorage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage2D", &["glTexStorage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexStorage2DMultisample {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexStorage2DMultisample.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexStorage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage2DMultisample", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexStorage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexStorage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexStorage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage3D", &["glTexStorage3DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexStorage3DMultisample {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexStorage3DMultisample.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexStorage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage3DMultisample", &["glTexStorage3DMultisampleOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage2D", &["glTexSubImage2DEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexSubImage3D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexSubImage3D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage3D", &["glTexSubImage3DEXT", "glTexSubImage3DOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TransformFeedbackVaryings {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TransformFeedbackVaryings.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::TransformFeedbackVaryings = FnPtr::new(metaloadfn(&mut loadfn, "glTransformFeedbackVaryings", &["glTransformFeedbackVaryingsEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1f", &["glUniform1fARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1fv", &["glUniform1fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1i", &["glUniform1iARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1iv", &["glUniform1ivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1ui", &["glUniform1uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform1uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform1uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1uiv", &["glUniform1uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2f", &["glUniform2fARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2fv", &["glUniform2fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2i", &["glUniform2iARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2iv", &["glUniform2ivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2ui", &["glUniform2uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform2uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform2uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2uiv", &["glUniform2uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3f", &["glUniform3fARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3fv", &["glUniform3fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3i", &["glUniform3iARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3iv", &["glUniform3ivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3ui", &["glUniform3uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform3uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform3uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3uiv", &["glUniform3uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4f", &["glUniform4fARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4fv", &["glUniform4fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4i", &["glUniform4iARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4iv", &["glUniform4ivARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4ui", &["glUniform4uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Uniform4uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Uniform4uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Uniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4uiv", &["glUniform4uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformBlockBinding {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformBlockBinding.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformBlockBinding = FnPtr::new(metaloadfn(&mut loadfn, "glUniformBlockBinding", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2fv", &["glUniformMatrix2fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix2x3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix2x3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x3fv", &["glUniformMatrix2x3fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix2x4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix2x4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x4fv", &["glUniformMatrix2x4fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3fv", &["glUniformMatrix3fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix3x2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix3x2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x2fv", &["glUniformMatrix3x2fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix3x4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix3x4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x4fv", &["glUniformMatrix3x4fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4fv", &["glUniformMatrix4fvARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix4x2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix4x2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x2fv", &["glUniformMatrix4x2fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UniformMatrix4x3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UniformMatrix4x3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x3fv", &["glUniformMatrix4x3fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UnmapBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UnmapBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UnmapBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glUnmapBuffer", &["glUnmapBufferARB", "glUnmapBufferOES"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UseProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UseProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UseProgram = FnPtr::new(metaloadfn(&mut loadfn, "glUseProgram", &["glUseProgramObjectARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UseProgramStages {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UseProgramStages.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::UseProgramStages = FnPtr::new(metaloadfn(&mut loadfn, "glUseProgramStages", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ValidateProgram {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ValidateProgram.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ValidateProgram = FnPtr::new(metaloadfn(&mut loadfn, "glValidateProgram", &["glValidateProgramARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ValidateProgramPipeline {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ValidateProgramPipeline.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::ValidateProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glValidateProgramPipeline", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib1f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib1f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib1f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1f", &["glVertexAttrib1fARB", "glVertexAttrib1fNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib1fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib1fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib1fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1fv", &["glVertexAttrib1fvARB", "glVertexAttrib1fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib2f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib2f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib2f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2f", &["glVertexAttrib2fARB", "glVertexAttrib2fNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib2fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib2fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib2fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2fv", &["glVertexAttrib2fvARB", "glVertexAttrib2fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib3f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib3f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3f", &["glVertexAttrib3fARB", "glVertexAttrib3fNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib3fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib3fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib3fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3fv", &["glVertexAttrib3fvARB", "glVertexAttrib3fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib4f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib4f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4f", &["glVertexAttrib4fARB", "glVertexAttrib4fNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttrib4fv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttrib4fv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttrib4fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4fv", &["glVertexAttrib4fvARB", "glVertexAttrib4fvNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribBinding {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribBinding.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribBinding = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribBinding", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribDivisor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribDivisor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribDivisor = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribDivisor", &["glVertexAttribDivisorANGLE", "glVertexAttribDivisorARB", "glVertexAttribDivisorEXT", "glVertexAttribDivisorNV"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribFormat {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribFormat.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribFormat", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribI4i {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribI4i.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribI4i = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4i", &["glVertexAttribI4iEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribI4iv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribI4iv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribI4iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4iv", &["glVertexAttribI4ivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribI4ui {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribI4ui.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribI4ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4ui", &["glVertexAttribI4uiEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribI4uiv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribI4uiv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribI4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4uiv", &["glVertexAttribI4uivEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribIFormat {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribIFormat.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribIFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribIFormat", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribIPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribIPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribIPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribIPointer", &["glVertexAttribIPointerEXT"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexAttribPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexAttribPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexAttribPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribPointer", &["glVertexAttribPointerARB"]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexBindingDivisor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexBindingDivisor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexBindingDivisor = FnPtr::new(metaloadfn(&mut loadfn, "glVertexBindingDivisor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Viewport {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Viewport.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::Viewport = FnPtr::new(metaloadfn(&mut loadfn, "glViewport", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod WaitSync {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::WaitSync.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
                    unsafe {
                        storage::WaitSync = FnPtr::new(metaloadfn(&mut loadfn, "glWaitSync", &["glWaitSyncAPPLE"]))
                    }
                }
            }
        
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gles2 function was not loaded")
        }
        

        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const __gl_imports::raw::c_void {
    
ActiveShaderProgram::load_with(&mut loadfn);
ActiveTexture::load_with(&mut loadfn);
AttachShader::load_with(&mut loadfn);
BeginQuery::load_with(&mut loadfn);
BeginTransformFeedback::load_with(&mut loadfn);
BindAttribLocation::load_with(&mut loadfn);
BindBuffer::load_with(&mut loadfn);
BindBufferBase::load_with(&mut loadfn);
BindBufferRange::load_with(&mut loadfn);
BindFramebuffer::load_with(&mut loadfn);
BindImageTexture::load_with(&mut loadfn);
BindProgramPipeline::load_with(&mut loadfn);
BindRenderbuffer::load_with(&mut loadfn);
BindSampler::load_with(&mut loadfn);
BindTexture::load_with(&mut loadfn);
BindTransformFeedback::load_with(&mut loadfn);
BindVertexArray::load_with(&mut loadfn);
BindVertexBuffer::load_with(&mut loadfn);
BlendBarrier::load_with(&mut loadfn);
BlendColor::load_with(&mut loadfn);
BlendEquation::load_with(&mut loadfn);
BlendEquationSeparate::load_with(&mut loadfn);
BlendEquationSeparatei::load_with(&mut loadfn);
BlendEquationi::load_with(&mut loadfn);
BlendFunc::load_with(&mut loadfn);
BlendFuncSeparate::load_with(&mut loadfn);
BlendFuncSeparatei::load_with(&mut loadfn);
BlendFunci::load_with(&mut loadfn);
BlitFramebuffer::load_with(&mut loadfn);
BufferData::load_with(&mut loadfn);
BufferSubData::load_with(&mut loadfn);
CheckFramebufferStatus::load_with(&mut loadfn);
Clear::load_with(&mut loadfn);
ClearBufferfi::load_with(&mut loadfn);
ClearBufferfv::load_with(&mut loadfn);
ClearBufferiv::load_with(&mut loadfn);
ClearBufferuiv::load_with(&mut loadfn);
ClearColor::load_with(&mut loadfn);
ClearDepthf::load_with(&mut loadfn);
ClearStencil::load_with(&mut loadfn);
ClientWaitSync::load_with(&mut loadfn);
ColorMask::load_with(&mut loadfn);
ColorMaski::load_with(&mut loadfn);
CompileShader::load_with(&mut loadfn);
CompressedTexImage2D::load_with(&mut loadfn);
CompressedTexImage3D::load_with(&mut loadfn);
CompressedTexSubImage2D::load_with(&mut loadfn);
CompressedTexSubImage3D::load_with(&mut loadfn);
CopyBufferSubData::load_with(&mut loadfn);
CopyImageSubData::load_with(&mut loadfn);
CopyTexImage2D::load_with(&mut loadfn);
CopyTexSubImage2D::load_with(&mut loadfn);
CopyTexSubImage3D::load_with(&mut loadfn);
CreateProgram::load_with(&mut loadfn);
CreateShader::load_with(&mut loadfn);
CreateShaderProgramv::load_with(&mut loadfn);
CullFace::load_with(&mut loadfn);
DebugMessageCallback::load_with(&mut loadfn);
DebugMessageControl::load_with(&mut loadfn);
DebugMessageInsert::load_with(&mut loadfn);
DeleteBuffers::load_with(&mut loadfn);
DeleteFramebuffers::load_with(&mut loadfn);
DeleteProgram::load_with(&mut loadfn);
DeleteProgramPipelines::load_with(&mut loadfn);
DeleteQueries::load_with(&mut loadfn);
DeleteRenderbuffers::load_with(&mut loadfn);
DeleteSamplers::load_with(&mut loadfn);
DeleteShader::load_with(&mut loadfn);
DeleteSync::load_with(&mut loadfn);
DeleteTextures::load_with(&mut loadfn);
DeleteTransformFeedbacks::load_with(&mut loadfn);
DeleteVertexArrays::load_with(&mut loadfn);
DepthFunc::load_with(&mut loadfn);
DepthMask::load_with(&mut loadfn);
DepthRangef::load_with(&mut loadfn);
DetachShader::load_with(&mut loadfn);
Disable::load_with(&mut loadfn);
DisableVertexAttribArray::load_with(&mut loadfn);
Disablei::load_with(&mut loadfn);
DispatchCompute::load_with(&mut loadfn);
DispatchComputeIndirect::load_with(&mut loadfn);
DrawArrays::load_with(&mut loadfn);
DrawArraysIndirect::load_with(&mut loadfn);
DrawArraysInstanced::load_with(&mut loadfn);
DrawBuffers::load_with(&mut loadfn);
DrawElements::load_with(&mut loadfn);
DrawElementsBaseVertex::load_with(&mut loadfn);
DrawElementsIndirect::load_with(&mut loadfn);
DrawElementsInstanced::load_with(&mut loadfn);
DrawElementsInstancedBaseVertex::load_with(&mut loadfn);
DrawRangeElements::load_with(&mut loadfn);
DrawRangeElementsBaseVertex::load_with(&mut loadfn);
Enable::load_with(&mut loadfn);
EnableVertexAttribArray::load_with(&mut loadfn);
Enablei::load_with(&mut loadfn);
EndQuery::load_with(&mut loadfn);
EndTransformFeedback::load_with(&mut loadfn);
FenceSync::load_with(&mut loadfn);
Finish::load_with(&mut loadfn);
Flush::load_with(&mut loadfn);
FlushMappedBufferRange::load_with(&mut loadfn);
FramebufferParameteri::load_with(&mut loadfn);
FramebufferRenderbuffer::load_with(&mut loadfn);
FramebufferTexture::load_with(&mut loadfn);
FramebufferTexture2D::load_with(&mut loadfn);
FramebufferTextureLayer::load_with(&mut loadfn);
FrontFace::load_with(&mut loadfn);
GenBuffers::load_with(&mut loadfn);
GenFramebuffers::load_with(&mut loadfn);
GenProgramPipelines::load_with(&mut loadfn);
GenQueries::load_with(&mut loadfn);
GenRenderbuffers::load_with(&mut loadfn);
GenSamplers::load_with(&mut loadfn);
GenTextures::load_with(&mut loadfn);
GenTransformFeedbacks::load_with(&mut loadfn);
GenVertexArrays::load_with(&mut loadfn);
GenerateMipmap::load_with(&mut loadfn);
GetActiveAttrib::load_with(&mut loadfn);
GetActiveUniform::load_with(&mut loadfn);
GetActiveUniformBlockName::load_with(&mut loadfn);
GetActiveUniformBlockiv::load_with(&mut loadfn);
GetActiveUniformsiv::load_with(&mut loadfn);
GetAttachedShaders::load_with(&mut loadfn);
GetAttribLocation::load_with(&mut loadfn);
GetBooleani_v::load_with(&mut loadfn);
GetBooleanv::load_with(&mut loadfn);
GetBufferParameteri64v::load_with(&mut loadfn);
GetBufferParameteriv::load_with(&mut loadfn);
GetBufferPointerv::load_with(&mut loadfn);
GetDebugMessageLog::load_with(&mut loadfn);
GetError::load_with(&mut loadfn);
GetFloatv::load_with(&mut loadfn);
GetFragDataLocation::load_with(&mut loadfn);
GetFramebufferAttachmentParameteriv::load_with(&mut loadfn);
GetFramebufferParameteriv::load_with(&mut loadfn);
GetGraphicsResetStatus::load_with(&mut loadfn);
GetInteger64i_v::load_with(&mut loadfn);
GetInteger64v::load_with(&mut loadfn);
GetIntegeri_v::load_with(&mut loadfn);
GetIntegerv::load_with(&mut loadfn);
GetInternalformativ::load_with(&mut loadfn);
GetMultisamplefv::load_with(&mut loadfn);
GetObjectLabel::load_with(&mut loadfn);
GetObjectPtrLabel::load_with(&mut loadfn);
GetPointerv::load_with(&mut loadfn);
GetProgramBinary::load_with(&mut loadfn);
GetProgramInfoLog::load_with(&mut loadfn);
GetProgramInterfaceiv::load_with(&mut loadfn);
GetProgramPipelineInfoLog::load_with(&mut loadfn);
GetProgramPipelineiv::load_with(&mut loadfn);
GetProgramResourceIndex::load_with(&mut loadfn);
GetProgramResourceLocation::load_with(&mut loadfn);
GetProgramResourceName::load_with(&mut loadfn);
GetProgramResourceiv::load_with(&mut loadfn);
GetProgramiv::load_with(&mut loadfn);
GetQueryObjectuiv::load_with(&mut loadfn);
GetQueryiv::load_with(&mut loadfn);
GetRenderbufferParameteriv::load_with(&mut loadfn);
GetSamplerParameterIiv::load_with(&mut loadfn);
GetSamplerParameterIuiv::load_with(&mut loadfn);
GetSamplerParameterfv::load_with(&mut loadfn);
GetSamplerParameteriv::load_with(&mut loadfn);
GetShaderInfoLog::load_with(&mut loadfn);
GetShaderPrecisionFormat::load_with(&mut loadfn);
GetShaderSource::load_with(&mut loadfn);
GetShaderiv::load_with(&mut loadfn);
GetString::load_with(&mut loadfn);
GetStringi::load_with(&mut loadfn);
GetSynciv::load_with(&mut loadfn);
GetTexLevelParameterfv::load_with(&mut loadfn);
GetTexLevelParameteriv::load_with(&mut loadfn);
GetTexParameterIiv::load_with(&mut loadfn);
GetTexParameterIuiv::load_with(&mut loadfn);
GetTexParameterfv::load_with(&mut loadfn);
GetTexParameteriv::load_with(&mut loadfn);
GetTransformFeedbackVarying::load_with(&mut loadfn);
GetUniformBlockIndex::load_with(&mut loadfn);
GetUniformIndices::load_with(&mut loadfn);
GetUniformLocation::load_with(&mut loadfn);
GetUniformfv::load_with(&mut loadfn);
GetUniformiv::load_with(&mut loadfn);
GetUniformuiv::load_with(&mut loadfn);
GetVertexAttribIiv::load_with(&mut loadfn);
GetVertexAttribIuiv::load_with(&mut loadfn);
GetVertexAttribPointerv::load_with(&mut loadfn);
GetVertexAttribfv::load_with(&mut loadfn);
GetVertexAttribiv::load_with(&mut loadfn);
GetnUniformfv::load_with(&mut loadfn);
GetnUniformiv::load_with(&mut loadfn);
GetnUniformuiv::load_with(&mut loadfn);
Hint::load_with(&mut loadfn);
InvalidateFramebuffer::load_with(&mut loadfn);
InvalidateSubFramebuffer::load_with(&mut loadfn);
IsBuffer::load_with(&mut loadfn);
IsEnabled::load_with(&mut loadfn);
IsEnabledi::load_with(&mut loadfn);
IsFramebuffer::load_with(&mut loadfn);
IsProgram::load_with(&mut loadfn);
IsProgramPipeline::load_with(&mut loadfn);
IsQuery::load_with(&mut loadfn);
IsRenderbuffer::load_with(&mut loadfn);
IsSampler::load_with(&mut loadfn);
IsShader::load_with(&mut loadfn);
IsSync::load_with(&mut loadfn);
IsTexture::load_with(&mut loadfn);
IsTransformFeedback::load_with(&mut loadfn);
IsVertexArray::load_with(&mut loadfn);
LineWidth::load_with(&mut loadfn);
LinkProgram::load_with(&mut loadfn);
MapBufferRange::load_with(&mut loadfn);
MemoryBarrier::load_with(&mut loadfn);
MemoryBarrierByRegion::load_with(&mut loadfn);
MinSampleShading::load_with(&mut loadfn);
ObjectLabel::load_with(&mut loadfn);
ObjectPtrLabel::load_with(&mut loadfn);
PatchParameteri::load_with(&mut loadfn);
PauseTransformFeedback::load_with(&mut loadfn);
PixelStorei::load_with(&mut loadfn);
PolygonOffset::load_with(&mut loadfn);
PopDebugGroup::load_with(&mut loadfn);
PrimitiveBoundingBox::load_with(&mut loadfn);
ProgramBinary::load_with(&mut loadfn);
ProgramParameteri::load_with(&mut loadfn);
ProgramUniform1f::load_with(&mut loadfn);
ProgramUniform1fv::load_with(&mut loadfn);
ProgramUniform1i::load_with(&mut loadfn);
ProgramUniform1iv::load_with(&mut loadfn);
ProgramUniform1ui::load_with(&mut loadfn);
ProgramUniform1uiv::load_with(&mut loadfn);
ProgramUniform2f::load_with(&mut loadfn);
ProgramUniform2fv::load_with(&mut loadfn);
ProgramUniform2i::load_with(&mut loadfn);
ProgramUniform2iv::load_with(&mut loadfn);
ProgramUniform2ui::load_with(&mut loadfn);
ProgramUniform2uiv::load_with(&mut loadfn);
ProgramUniform3f::load_with(&mut loadfn);
ProgramUniform3fv::load_with(&mut loadfn);
ProgramUniform3i::load_with(&mut loadfn);
ProgramUniform3iv::load_with(&mut loadfn);
ProgramUniform3ui::load_with(&mut loadfn);
ProgramUniform3uiv::load_with(&mut loadfn);
ProgramUniform4f::load_with(&mut loadfn);
ProgramUniform4fv::load_with(&mut loadfn);
ProgramUniform4i::load_with(&mut loadfn);
ProgramUniform4iv::load_with(&mut loadfn);
ProgramUniform4ui::load_with(&mut loadfn);
ProgramUniform4uiv::load_with(&mut loadfn);
ProgramUniformMatrix2fv::load_with(&mut loadfn);
ProgramUniformMatrix2x3fv::load_with(&mut loadfn);
ProgramUniformMatrix2x4fv::load_with(&mut loadfn);
ProgramUniformMatrix3fv::load_with(&mut loadfn);
ProgramUniformMatrix3x2fv::load_with(&mut loadfn);
ProgramUniformMatrix3x4fv::load_with(&mut loadfn);
ProgramUniformMatrix4fv::load_with(&mut loadfn);
ProgramUniformMatrix4x2fv::load_with(&mut loadfn);
ProgramUniformMatrix4x3fv::load_with(&mut loadfn);
PushDebugGroup::load_with(&mut loadfn);
ReadBuffer::load_with(&mut loadfn);
ReadPixels::load_with(&mut loadfn);
ReadnPixels::load_with(&mut loadfn);
ReleaseShaderCompiler::load_with(&mut loadfn);
RenderbufferStorage::load_with(&mut loadfn);
RenderbufferStorageMultisample::load_with(&mut loadfn);
ResumeTransformFeedback::load_with(&mut loadfn);
SampleCoverage::load_with(&mut loadfn);
SampleMaski::load_with(&mut loadfn);
SamplerParameterIiv::load_with(&mut loadfn);
SamplerParameterIuiv::load_with(&mut loadfn);
SamplerParameterf::load_with(&mut loadfn);
SamplerParameterfv::load_with(&mut loadfn);
SamplerParameteri::load_with(&mut loadfn);
SamplerParameteriv::load_with(&mut loadfn);
Scissor::load_with(&mut loadfn);
ShaderBinary::load_with(&mut loadfn);
ShaderSource::load_with(&mut loadfn);
StencilFunc::load_with(&mut loadfn);
StencilFuncSeparate::load_with(&mut loadfn);
StencilMask::load_with(&mut loadfn);
StencilMaskSeparate::load_with(&mut loadfn);
StencilOp::load_with(&mut loadfn);
StencilOpSeparate::load_with(&mut loadfn);
TexBuffer::load_with(&mut loadfn);
TexBufferRange::load_with(&mut loadfn);
TexImage2D::load_with(&mut loadfn);
TexImage3D::load_with(&mut loadfn);
TexParameterIiv::load_with(&mut loadfn);
TexParameterIuiv::load_with(&mut loadfn);
TexParameterf::load_with(&mut loadfn);
TexParameterfv::load_with(&mut loadfn);
TexParameteri::load_with(&mut loadfn);
TexParameteriv::load_with(&mut loadfn);
TexStorage2D::load_with(&mut loadfn);
TexStorage2DMultisample::load_with(&mut loadfn);
TexStorage3D::load_with(&mut loadfn);
TexStorage3DMultisample::load_with(&mut loadfn);
TexSubImage2D::load_with(&mut loadfn);
TexSubImage3D::load_with(&mut loadfn);
TransformFeedbackVaryings::load_with(&mut loadfn);
Uniform1f::load_with(&mut loadfn);
Uniform1fv::load_with(&mut loadfn);
Uniform1i::load_with(&mut loadfn);
Uniform1iv::load_with(&mut loadfn);
Uniform1ui::load_with(&mut loadfn);
Uniform1uiv::load_with(&mut loadfn);
Uniform2f::load_with(&mut loadfn);
Uniform2fv::load_with(&mut loadfn);
Uniform2i::load_with(&mut loadfn);
Uniform2iv::load_with(&mut loadfn);
Uniform2ui::load_with(&mut loadfn);
Uniform2uiv::load_with(&mut loadfn);
Uniform3f::load_with(&mut loadfn);
Uniform3fv::load_with(&mut loadfn);
Uniform3i::load_with(&mut loadfn);
Uniform3iv::load_with(&mut loadfn);
Uniform3ui::load_with(&mut loadfn);
Uniform3uiv::load_with(&mut loadfn);
Uniform4f::load_with(&mut loadfn);
Uniform4fv::load_with(&mut loadfn);
Uniform4i::load_with(&mut loadfn);
Uniform4iv::load_with(&mut loadfn);
Uniform4ui::load_with(&mut loadfn);
Uniform4uiv::load_with(&mut loadfn);
UniformBlockBinding::load_with(&mut loadfn);
UniformMatrix2fv::load_with(&mut loadfn);
UniformMatrix2x3fv::load_with(&mut loadfn);
UniformMatrix2x4fv::load_with(&mut loadfn);
UniformMatrix3fv::load_with(&mut loadfn);
UniformMatrix3x2fv::load_with(&mut loadfn);
UniformMatrix3x4fv::load_with(&mut loadfn);
UniformMatrix4fv::load_with(&mut loadfn);
UniformMatrix4x2fv::load_with(&mut loadfn);
UniformMatrix4x3fv::load_with(&mut loadfn);
UnmapBuffer::load_with(&mut loadfn);
UseProgram::load_with(&mut loadfn);
UseProgramStages::load_with(&mut loadfn);
ValidateProgram::load_with(&mut loadfn);
ValidateProgramPipeline::load_with(&mut loadfn);
VertexAttrib1f::load_with(&mut loadfn);
VertexAttrib1fv::load_with(&mut loadfn);
VertexAttrib2f::load_with(&mut loadfn);
VertexAttrib2fv::load_with(&mut loadfn);
VertexAttrib3f::load_with(&mut loadfn);
VertexAttrib3fv::load_with(&mut loadfn);
VertexAttrib4f::load_with(&mut loadfn);
VertexAttrib4fv::load_with(&mut loadfn);
VertexAttribBinding::load_with(&mut loadfn);
VertexAttribDivisor::load_with(&mut loadfn);
VertexAttribFormat::load_with(&mut loadfn);
VertexAttribI4i::load_with(&mut loadfn);
VertexAttribI4iv::load_with(&mut loadfn);
VertexAttribI4ui::load_with(&mut loadfn);
VertexAttribI4uiv::load_with(&mut loadfn);
VertexAttribIFormat::load_with(&mut loadfn);
VertexAttribIPointer::load_with(&mut loadfn);
VertexAttribPointer::load_with(&mut loadfn);
VertexBindingDivisor::load_with(&mut loadfn);
Viewport::load_with(&mut loadfn);
WaitSync::load_with(&mut loadfn);

        }
    
