use super::*;

pub fn ui_shader() -> Shader {
    const VSCODE: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec2 texCoords;
        // we technically don't need normals but I want this to be compatible with models
        layout (location = 2) in vec3 normals;
        uniform mat4 view;
        out vec2 frag_texcoords;
        void main() {
            frag_texcoords = texCoords;
            gl_Position = view * vec4(pos, 1.0);
        }"#;
    const FSCODE: &str = r#"#version 330 core
        out vec4 final_color;
        in vec2 frag_texcoords;
        uniform sampler2D textur;
        void main() {
            final_color = texture(textur, frag_texcoords);
            final_color.w = 1.0;
        }"#;
    unsafe { Shader::new(VSCODE, FSCODE) }
}

pub fn game_shader() -> Shader {
    const VSCODE: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec2 texCoords;
        layout (location = 2) in vec3 normals;
        uniform mat4 view;
        uniform mat4 model;
        out vec2 frag_texcoords;
        out vec3 frag_normals;
        void main() {
            frag_texcoords = texCoords;
            frag_normals = normalize(normals);
            gl_Position = view * model * vec4(pos, 1.0);
        }"#;
    const FSCODE: &str = r#"#version 330 core
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
        }"#;
    unsafe { Shader::new(VSCODE, FSCODE) }
}
