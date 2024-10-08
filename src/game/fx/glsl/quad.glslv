#version 300 es
layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec2 a_translate;
layout(location = 3) in vec2 a_scale;
layout(location = 4) in float a_layer;
layout(location = 5) in float a_rotate;
layout(location = 6) in float a_tex_slot;
layout(location = 7) in float a_opacity;

layout (std140) uniform Locals {
	mat4 u_view_proj;
};

out vec3 v_tex_coord;
out float v_opacity;

vec2 world(vec2 pos, vec2 trans, vec2 scale, float rot) {
	float s = sin(rot);
	float c = cos(rot);
	mat2 mat_rot = mat2(c, s, -s, c);
	return mat_rot * (pos * scale) + trans;
}

void main() {
    v_tex_coord = vec3(a_tex_coord, a_tex_slot);
	v_opacity = a_opacity;
    gl_Position = u_view_proj * vec4(world(a_pos, a_translate, a_scale, a_rotate), a_layer, 1.0);
}