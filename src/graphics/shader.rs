use super::*;

#[allow(unused)]
pub enum Uniform {
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4(Mat4),
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub unsafe fn new(vscode: &str, fscode: &str) -> Self {
        let vs = glCreateShader(gl33::GL_VERTEX_SHADER);
        assert_ne!(vs, 0);
        glShaderSource(
            vs,
            1,
            &(vscode.as_bytes().as_ptr().cast()),
            &(vscode.len().try_into().unwrap()),
        );
        glCompileShader(vs);
        let mut success = 0;
        glGetShaderiv(vs, gl33::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut cstring: Vec<u8> = Vec::with_capacity(1024);
            let mut len: i32 = 0;
            glGetShaderInfoLog(vs, 1024, &mut len, cstring.as_mut_ptr().cast());
            cstring.set_len(len.try_into().unwrap());
            panic!("Vertex shader error: {}", String::from_utf8_lossy(&cstring));
        }

        let fs = glCreateShader(gl33::GL_FRAGMENT_SHADER);
        assert_ne!(fs, 0);
        glShaderSource(
            fs,
            1,
            &(fscode.as_bytes().as_ptr().cast()),
            &(fscode.len().try_into().unwrap()),
        );
        glCompileShader(fs);
        let mut success = 0;
        glGetShaderiv(fs, gl33::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut cstring: Vec<u8> = Vec::with_capacity(1024);
            let mut len: i32 = 0;
            glGetShaderInfoLog(fs, 1024, &mut len, cstring.as_mut_ptr().cast());
            cstring.set_len(len.try_into().unwrap());
            panic!(
                "Fragment shader error: {}",
                String::from_utf8_lossy(&cstring)
            );
        }
        let program = glCreateProgram();
        glAttachShader(program, vs);
        glAttachShader(program, fs);
        glLinkProgram(program);
        let mut success = 0;
        glGetProgramiv(program, gl33::GL_LINK_STATUS, &mut success);
        if success == 0 {
            panic!("Unable to link shader program");
        }
        glDeleteShader(vs);
        glDeleteShader(fs);

        Shader { id: program }
    }

    pub fn create_uniform(&self, name: &str) -> i32 {
        unsafe {
            let mut name = String::from(name);
            name += "\0";
            glGetUniformLocation(self.id, name.as_bytes().as_ptr() as _)
        }
    }

    pub fn set_uniform(&self, id: i32, value: Uniform) {
        self.bind();
        unsafe {
            match value {
                Uniform::Int(x) => glUniform1i(id, x),
                Uniform::Float(x) => glUniform1f(id, x),
                Uniform::Vec2(x) => glUniform2f(id, x.x, x.y),
                Uniform::Vec3(x) => glUniform3f(id, x.x, x.y, x.z),
                Uniform::Mat4(x) => {
                    glUniformMatrix4fv(id, 1, false as _, x.to_cols_array().as_ptr() as _)
                }
            }
        }
    }

    // no unbind function, since no shader crashes opengl
    pub fn bind(&self) {
        glUseProgram(self.id);
    }
}
