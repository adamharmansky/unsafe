use crate::graphics::{shader, Shader};
use glam::Mat4;
use glam::Vec3;

pub struct RenderView {
    shader: Shader,
    view_matrix: i32,
    model_matrix: i32,
    camera_position: i32,
}

impl RenderView {
    pub fn new(shader: Shader) -> Self {
        let view_matrix = shader.create_uniform("view");
        let model_matrix = shader.create_uniform("model");
        let camera_position = shader.create_uniform("camera_position");
        Self {
            shader,
            view_matrix,
            model_matrix,
            camera_position,
        }
    }

    pub fn set_view(&mut self, matrix: Mat4) {
        self.shader
            .set_uniform(self.view_matrix, shader::Uniform::Mat4(matrix));
    }
    pub fn set_model(&mut self, matrix: Mat4) {
        self.shader
            .set_uniform(self.model_matrix, shader::Uniform::Mat4(matrix));
    }
    pub fn set_camera_position(&mut self, matrix: Vec3) {
        self.shader
            .set_uniform(self.camera_position, shader::Uniform::Vec3(matrix));
    }

    pub fn bind(&mut self) {
        self.shader.bind();
    }
}
