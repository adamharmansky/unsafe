use super::*;

pub struct Model {
    vao: u32,
    vertices: u32,
    indices: u32,
    texcoords: u32,
    normals: u32,
    count: i32,
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &self.vertices);
            glDeleteBuffers(1, &self.indices);
            glDeleteBuffers(1, &self.texcoords);
            glDeleteBuffers(1, &self.normals);
            glDeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Renderable for Model {
    fn render(&self) {
        unsafe {
            glBindVertexArray(self.vao);
            glDrawElements(
                gl33::GL_TRIANGLES,
                self.count,
                gl33::GL_UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}

impl Model {
    pub fn new(data: &MeshData) -> Self {
        let mut id: u32 = 0;
        let mut vertices: u32 = 0;
        let mut indices: u32 = 0;
        let mut texcoords: u32 = 0;
        let mut normals: u32 = 0;
        unsafe {
            // the VAO which will hold all of our VBOs
            glGenVertexArrays(1, &mut id);
            assert_ne!(id, 0);
            glBindVertexArray(id);

            // vertices
            glGenBuffers(1, &mut vertices);
            assert_ne!(vertices, 0);
            glBindBuffer(gl33::GL_ARRAY_BUFFER, vertices);

            glBufferData(
                gl33::GL_ARRAY_BUFFER,
                (data.vertices.len() * std::mem::size_of::<(f32, f32, f32)>()) as isize,
                data.vertices.as_ptr() as *const std::ffi::c_void,
                gl33::GL_STATIC_DRAW,
            );
            glVertexAttribPointer(
                0,
                3,
                gl33::GL_FLOAT,
                0,
                std::mem::size_of::<(f32, f32, f32)>().try_into().unwrap(),
                0 as *const _,
            );
            glEnableVertexAttribArray(0);

            // texture coordinates
            glGenBuffers(1, &mut texcoords);
            assert_ne!(texcoords, 0);
            glBindBuffer(gl33::GL_ARRAY_BUFFER, texcoords);
            glBufferData(
                gl33::GL_ARRAY_BUFFER,
                (data.texcoords.len() * std::mem::size_of::<(f32, f32)>()) as isize,
                data.texcoords.as_ptr() as *const std::ffi::c_void,
                gl33::GL_STATIC_DRAW,
            );
            glVertexAttribPointer(
                1,
                2,
                gl33::GL_FLOAT,
                0,
                std::mem::size_of::<(f32, f32)>().try_into().unwrap(),
                0 as *const _,
            );
            glEnableVertexAttribArray(1);

            // normals
            glGenBuffers(1, &mut normals);
            assert_ne!(normals, 0);
            glBindBuffer(gl33::GL_ARRAY_BUFFER, normals);
            glBufferData(
                gl33::GL_ARRAY_BUFFER,
                (data.normals.len() * std::mem::size_of::<(f32, f32, f32)>()) as isize,
                data.normals.as_ptr() as *const std::ffi::c_void,
                gl33::GL_STATIC_DRAW,
            );
            glVertexAttribPointer(
                2,
                3,
                gl33::GL_FLOAT,
                0,
                std::mem::size_of::<(f32, f32, f32)>().try_into().unwrap(),
                0 as *const _,
            );
            glEnableVertexAttribArray(2);

            // indices
            glGenBuffers(1, &mut indices);
            assert_ne!(indices, 0);
            glBindBuffer(gl33::GL_ELEMENT_ARRAY_BUFFER, indices);

            glBufferData(
                gl33::GL_ELEMENT_ARRAY_BUFFER,
                (data.indices.len() * std::mem::size_of::<(i32, i32, i32)>()) as isize,
                data.indices.as_ptr() as *const std::ffi::c_void,
                gl33::GL_STATIC_DRAW,
            );
        }
        Model {
            vao: id,
            vertices,
            indices,
            texcoords,
            normals,
            count: (data.indices.len() * 3) as i32,
        }
    }
}
