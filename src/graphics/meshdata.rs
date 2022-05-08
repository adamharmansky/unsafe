#[derive(Debug)]
pub struct MeshData {
    pub vertices: Vec<(f32, f32, f32)>,
    pub indices: Vec<(i32, i32, i32)>,
    pub texcoords: Vec<(f32, f32)>,
}

#[allow(unused)]
impl MeshData {
    pub fn append(&mut self, mut other: Self) {
        let index_offset: i32 = self.vertices.len() as i32;
        self.vertices.append(&mut other.vertices);
        for i in other.indices {
            self.indices
                .push((i.0 + index_offset, i.1 + index_offset, i.2 + index_offset));
        }
        self.texcoords.append(&mut other.texcoords);
    }

    pub fn translate(&mut self, pos: (f32, f32, f32)) {
        for mut i in &mut self.vertices {
            i.0 += pos.0;
            i.1 += pos.1;
            i.2 += pos.2;
        }
    }

    pub fn new() -> Self {
        MeshData {
            vertices: Vec::new(),
            indices: Vec::new(),
            texcoords: Vec::new(),
        }
    }

    pub fn cube() -> Self {
        MeshData {
            vertices: vec![
                // sides
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (1.0, 1.0, 0.0),
                (0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (0.0, 0.0, 1.0),
                (0.0, 1.0, 1.0),
                (1.0, 0.0, 1.0),
                (0.0, 0.0, 1.0),
                (1.0, 1.0, 1.0),
                (0.0, 1.0, 1.0),
                (1.0, 0.0, 0.0),
                (1.0, 0.0, 1.0),
                (1.0, 1.0, 0.0),
                (1.0, 1.0, 1.0),
                //top
                (0.0, 1.0, 0.0),
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 1.0),
                (1.0, 1.0, 1.0),
                //bottom
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0),
                (1.0, 0.0, 1.0),
            ],
            indices: vec![
                (0, 1, 2),
                (1, 2, 3),
                (4, 5, 6),
                (5, 6, 7),
                (8, 9, 10),
                (9, 10, 11),
                (12, 13, 14),
                (13, 14, 15),
                (16, 17, 18),
                (17, 18, 19),
                (20, 21, 22),
                (21, 22, 23),
            ],
            texcoords: vec![
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
            ],
        }
    }
}
