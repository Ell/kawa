#version 410

uniform mat4 u_prj_matrix;

in vec2 position;
in vec2 uv;
in vec4 color;

out vec2 frag_uv;
out vec4 frag_color;

void main() {
    frag_uv = uv;
    frag_color = color;

    gl_Position = u_prj_matrix * vec4(position.xy, 0, 1);
}