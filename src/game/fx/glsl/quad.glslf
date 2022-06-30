#version 300 es
precision mediump float;
precision mediump sampler2DArray;

in vec3 v_tex_coord;
in float v_opacity;

uniform sampler2DArray t_textures;

layout(location = 0) out vec4 out_target;

void main() {
    vec4 color = texture(t_textures, v_tex_coord);
    out_target = vec4(color.rgb, color.a * v_opacity);
}