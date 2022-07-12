#version 330 core
out vec4 final_color;
in vec2 frag_texcoords;
uniform sampler2D textur;
void main() {
    final_color = texture(textur, frag_texcoords);
    final_color.w = 1.0;
}
