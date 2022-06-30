#version 300 es
layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec2 a_translate;
layout(location = 3) in vec2 a_scale;
layout(location = 4) in vec4 a_color;
layout(location = 5) in float a_layer;
layout(location = 6) in float a_tex_slot;
layout(location = 7) in float a_width;

layout (std140) uniform Locals {
	mat4 u_view_proj;
};

out vec3 v_tex_coord;
out vec4 v_color;

void main() {
    float offset = (1.0 - a_width) * 0.5;
    v_color = a_color;
    v_tex_coord = vec3(abs(a_tex_coord.x - offset), a_tex_coord.y, a_tex_slot);
    gl_Position = u_view_proj * vec4(a_pos * a_scale + a_translate, a_layer, 1.0);
}