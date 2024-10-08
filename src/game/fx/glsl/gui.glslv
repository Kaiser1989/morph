#version 300 es
layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tex_coord;
layout(location = 2) in vec2 a_translate;
layout(location = 3) in vec2 a_scale;
layout(location = 4) in vec4 a_color;
layout(location = 5) in float a_layer;
layout(location = 6) in float a_tex_slot;
layout(location = 7) in float a_slice;
layout(location = 8) in float a_radius;

layout (std140) uniform Locals {
	mat4 u_view_proj;
};

out vec3 v_tex_coord;
out vec3 v_slice_coord;
out vec4 v_color;

#define M_PI 3.1415926535897932384626433832795

vec2 world(vec2 pos, vec2 trans, vec2 scale, float rot) {
	float s = sin(rot);
	float c = cos(rot);
	mat2 mat_rot = mat2(c, s, -s, c);
	return mat_rot * (pos * scale) + trans;
}

void main() {
	// calc slice info
	vec2 zero = vec2(0.0);
	vec2 radius = vec2(a_radius);
	vec2 n = vec2(floor(a_slice / 3.0), mod(a_slice, 3.0)) - vec2(1.0);

	// calc 9 slice pos, scale, coords
	vec2 uv = radius / a_scale;
	vec2 tex_coord = a_tex_coord * mix(1.0 - 2.0 * uv, uv, abs(n)) + mix(uv * (1.0 + min(n, zero)), 1.0 - uv, max(n, zero));
	vec2 scale = mix(a_scale - 2.0 * radius, radius, abs(n));
	vec2 translate = a_translate + (a_scale - radius) * n;
	float slice_slot = abs(n.x * n.y);
	float rotate = slice_slot * M_PI * 0.25 * abs(n.x + 3.0 * n.y - 2.0 * n.x * n.y);

	// set data
    v_tex_coord = vec3(tex_coord, a_tex_slot);
	v_slice_coord = vec3(a_tex_coord, slice_slot);
    v_color = a_color;
    gl_Position = u_view_proj * vec4(world(a_pos, translate, scale, rotate), a_layer, 1.0);
}