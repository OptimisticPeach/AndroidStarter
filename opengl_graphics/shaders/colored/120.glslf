#version 310 es
precision mediump float;

in vec4 v_Color;
out vec4 outColor;

void main() {
    outColor = v_Color;
}