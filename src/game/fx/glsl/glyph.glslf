#version 300 es
precision mediump float;
precision mediump sampler2DArray;

in vec3 v_tex_coord;
in vec4 v_color;

uniform sampler2DArray t_glyphs;

layout(location = 0) out vec4 out_target;

void main() {
    out_target = vec4(1.0, 1.0, 1.0, texture(t_glyphs, v_tex_coord).r) * v_color;
}