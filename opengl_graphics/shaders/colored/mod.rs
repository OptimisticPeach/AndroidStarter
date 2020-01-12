//! Shaders for colored rendering.

/// Vertex shader for GLSL 1.20
pub const VERTEX_GLSL_120: &'static [u8] = include_bytes!("120.glslv");

/// Fragment shader for GLSL 1.20
pub const FRAGMENT_GLSL_120: &'static [u8] = include_bytes!("120.glslf");