#version 330 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 texCoords;
// we technically don't need normals but I want this to be compatible with models
layout (location = 2) in vec3 normals;
uniform mat4 view;
uniform mat4 model;
out vec2 frag_texcoords;
void main() {
    frag_texcoords = texCoords;
    gl_Position = view * model * vec4(pos, 1.0);
}
