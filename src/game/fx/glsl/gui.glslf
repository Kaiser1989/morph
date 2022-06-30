#version 300 es
precision mediump float;
precision mediump sampler2DArray;

in vec3 v_tex_coord;
in vec3 v_slice_coord;
in vec4 v_color;

uniform sampler2DArray t_textures;
uniform sampler2DArray t_slices;

layout(location = 0) out vec4 out_target;

void main() {
    out_target = texture(t_textures, v_tex_coord) * texture(t_slices, v_slice_coord) * v_color;
}