#version 310 es
precision mediump float;
uniform sampler2D s_texture;
uniform vec4 color;

in vec2 v_UV;
// varying vec2 v_UV;

out vec4 outColor;

void main()
{
    outColor = texture(s_texture, v_UV) * color;
}