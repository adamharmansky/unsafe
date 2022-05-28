use super::*;
use meshdata::MeshData;

pub struct Chunk {
    pub pos: BlockPos,
    blocks: [[[BlockID; 16]; 16]; 16],
    pub model: Option<Model>,
}

unsafe impl Send for Chunk {}

impl Chunk {
    /// Creates a new chunk, without a mesh
    pub fn new(pos: BlockPos, manager: &BlockManager) -> Self {
        let px = pos.x * 16;
        let py = pos.y * 16;
        let pz = pos.z * 16;
        Chunk {
            pos,
            model: None,
            blocks: {
                let mut data: [[[BlockID; 16]; 16]; 16] = [[[0; 16]; 16]; 16];
                for i in 0..16 {
                    for j in 0..16 {
                        for k in 0..16 {
                            let x: i32 = px + i;
                            let y: i32 = py + k;
                            let z: i32 = pz + j;
                            if (y as f32)
                                < ((x as f32 / 2.0) - (z as f32 / 4.0)).sin()
                                    - 2.0 * ((x as f32 / 3.0) + (z as f32 / 80.0)).sin()
                                    + (z as f32 / 3.0).sin()
                                    - ((z as f32 / 2.0) + (x as f32 / 4.0) + 1.0).sin()
                                    + 6.0 * ((x as f32 / 12.0).sin() + (z as f32 / 9.0).cos()).sin()
                            {
                                if y >= 0 {
                                    data[i as usize][k as usize][j as usize] =
                                        manager[String::from("grass")];
                                } else {
                                    data[i as usize][k as usize][j as usize] =
                                        manager[String::from("stone")];
                                }
                            } else {
                                data[i as usize][k as usize][j as usize] =
                                    manager[String::from("air")];
                            }
                        }
                    }
                }
                data
            },
        }
    }

    /// Get a block, None if outside of chunk
    pub fn get_block(&self, pos: BlockPos) -> Option<BlockID> {
        if pos.is_in_chunk() {
            Some(self.blocks[pos.x as usize][pos.y as usize][pos.z as usize])
        } else {
            None
        }
    }

    /// Set a block, panics if outside a chunk
    pub fn set_block(&mut self, pos: BlockPos, b: BlockID) {
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = b;
    }

    /// Render if model present
    pub fn render(&self) {
        if let Some(x) = &self.model {
            x.render();
        }
    }

    /// Updates the chunk (generates a mesh)
    ///
    /// Takes a couple of milliseconds
    pub fn update(&mut self, blocks: &BlockManager) -> MeshData {
        let mut data = MeshData::new();
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    (blocks[self.blocks[i][j][k]].gen_mesh)(
                        &mut data,
                        BlockPos::new(
                            self.pos.x * 16 + i as i32,
                            self.pos.y * 16 + j as i32,
                            self.pos.z * 16 + k as i32,
                        ),
                        util::BlockSides {
                            top: if j == 15 {
                                true
                            } else {
                                !blocks[self.blocks[i][j + 1][k]].solid
                            },
                            bottom: if j == 0 {
                                true
                            } else {
                                !blocks[self.blocks[i][j - 1][k]].solid
                            },
                            left: if i == 0 {
                                true
                            } else {
                                !blocks[self.blocks[i - 1][j][k]].solid
                            },
                            right: if i == 15 {
                                true
                            } else {
                                !blocks[self.blocks[i + 1][j][k]].solid
                            },
                            front: if k == 15 {
                                true
                            } else {
                                !blocks[self.blocks[i][j][k + 1]].solid
                            },
                            back: if k == 0 {
                                true
                            } else {
                                !blocks[self.blocks[i][j][k - 1]].solid
                            },
                        },
                    );
                }
            }
        }
        data
    }
}
