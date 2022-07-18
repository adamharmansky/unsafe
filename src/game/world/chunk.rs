use super::*;
use meshdata::MeshData;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

pub struct Chunk {
    pub pos: BlockPos,
    pub model: Option<Model>,
    blocks: [[[BlockID; 16]; 16]; 16],

    filename: String,
    save_dir: String,
    modified: bool,
}

unsafe impl Send for Chunk {}

impl Drop for Chunk {
    fn drop(&mut self) {
        if self.modified {
            self.save().unwrap();
        }
    }
}

impl Chunk {
    /// Creates a new chunk, without a mesh
    pub fn new(pos: BlockPos, manager: &BlockManager, save_dir: &str) -> Self {
        let mut chunk = Chunk {
            pos,
            model: None,
            blocks: [[[0; 16]; 16]; 16],
            filename: format!("{}/{}-{}-{}.chunk", save_dir, pos.x, pos.y, pos.z),
            modified: false,
            save_dir: String::from(save_dir),
        };
        if let Err(_) = chunk.load() {
            chunk.generate(manager);
        }
        chunk
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let file = File::open(&self.filename)?;
        let mut file = BufReader::new(file);
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let mut buf: [u8; 1] = [0];
                    file.read(&mut buf)?;
                    self.blocks[i][j][k] = buf[0] as BlockID;
                }
            }
        }
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Ok(_) = std::fs::create_dir(self.save_dir.as_str()) {
            println!("creating a new save!");
        }
        let file = File::create(&self.filename)?;
        let mut file = BufWriter::new(file);
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let x = [self.blocks[i][j][k] as u8];
                    file.write(&x)?;
                }
            }
        }
        Ok(())
    }

    pub fn generate(&mut self, manager: &BlockManager) {
        let px = self.pos.x * 16;
        let py = self.pos.y * 16;
        let pz = self.pos.z * 16;
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let x: i32 = px + i;
                    let y: i32 = py + k;
                    let z: i32 = pz + j;
                    let h = ((x as f32 / 2.0) - (z as f32 / 4.0)).sin()
                        - 2.0 * ((x as f32 / 3.0) + (z as f32 / 80.0)).sin()
                        + (z as f32 / 3.0).sin()
                        - ((z as f32 / 2.0) + (x as f32 / 4.0) + 1.0).sin()
                        + 6.0 * ((x as f32 / 12.0).sin() + (z as f32 / 9.0).cos()).sin();
                    let hd = y - (h.floor() as i32);
                    if hd > 0 {
                        self.blocks[i as usize][k as usize][j as usize] =
                            manager[String::from("air")];
                    } else if hd == 0 {
                        self.blocks[i as usize][k as usize][j as usize] =
                            manager[String::from("grass")];
                    } else if hd > -5 {
                        self.blocks[i as usize][k as usize][j as usize] =
                            manager[String::from("dirt")];
                    } else {
                        self.blocks[i as usize][k as usize][j as usize] =
                            manager[String::from("stone")];
                    }
                }
            }
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
        self.modified = true;
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
    pub fn update(&mut self, blocks: Arc<BlockManager>, world: &mut ChunkServer) {
        let mut data = MeshData::new();
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let p = BlockPos::new(
                        self.pos.x * 16 + i as i32,
                        self.pos.y * 16 + j as i32,
                        self.pos.z * 16 + k as i32,
                    );
                    (blocks[self.blocks[i][j][k]].gen_mesh)(
                        &mut data,
                        p,
                        util::BlockSides {
                            top: if j == 15 {
                                !blocks[world.get_block(p + BlockPos::new(0, 1, 0)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i][j + 1][k]].solid
                            },
                            bottom: if j == 0 {
                                !blocks[world.get_block(p + BlockPos::new(0, -1, 0)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i][j - 1][k]].solid
                            },
                            left: if i == 0 {
                                !blocks[world.get_block(p + BlockPos::new(-1, 0, 0)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i - 1][j][k]].solid
                            },
                            right: if i == 15 {
                                !blocks[world.get_block(p + BlockPos::new(1, 0, 0)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i + 1][j][k]].solid
                            },
                            front: if k == 15 {
                                !blocks[world.get_block(p + BlockPos::new(0, 0, 1)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i][j][k + 1]].solid
                            },
                            back: if k == 0 {
                                !blocks[world.get_block(p + BlockPos::new(0, 0, -1)).unwrap()].solid
                            } else {
                                !blocks[self.blocks[i][j][k - 1]].solid
                            },
                        },
                    );
                }
            }
        }
        self.model = None;
        self.model = Some(Model::new(&data));
    }
}
