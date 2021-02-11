#version 410

uniform sampler2D u_texture;

in vec2 frag_uv;
in vec4 frag_color;

out vec4 out_color;

void main() {
  out_color = frag_color * texture(u_texture, frag_uv.st);
}