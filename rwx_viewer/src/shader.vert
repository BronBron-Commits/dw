#version 330 core
layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec2 in_uv;

uniform mat4 u_mvp;

out vec2 v_uv;

void main() {
    v_uv = in_uv;
    gl_Position = u_mvp * vec4(in_pos, 1.0);
}
