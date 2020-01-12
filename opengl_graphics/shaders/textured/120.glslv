#version 310 es 

uniform sampler2D s_texture;
uniform vec4 color;

in vec2 pos;
in vec2 uv;
// attribute vec2 pos;
// attribute vec2 uv;

out vec2 v_UV;
// varying vec2 v_UV;

void main() {
    v_UV = uv;
    gl_Position = vec4(pos, 0.0, 1.0);
}