#version 330 core
out vec4 final_color;
in vec2 frag_texcoords;
in vec3 frag_normals;
uniform sampler2D textur;
void main() {
    // final_color = texture(textur, frag_texcoords);
    float light = dot(frag_normals, normalize(vec3(1.0, 2.0, -1.0)));
    light = light / 8.0 + 0.85;
    final_color = texture(textur, frag_texcoords) * light;
    final_color.w = 1.0;
}
