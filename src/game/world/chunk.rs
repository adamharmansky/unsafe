use super::*;
use block::{Air, Block, Grass, Stone};
use meshdata::MeshData;
use std::mem::{transmute, MaybeUninit};
use std::sync::mpsc;
use std::thread;

pub struct Chunk {
    pub pos: BlockPos,
    blocks: [[[Box<dyn Block>; 16]; 16]; 16],
    pub model: Option<Model>,
}

unsafe impl Send for Chunk {}

impl Chunk {
    /// Creates a new chunk, without a mesh
    pub fn new(pos: BlockPos) -> Self {
        let px = pos.x * 16;
        let py = pos.y * 16;
        let pz = pos.z * 16;
        Chunk {
            pos,
            model: None,
            blocks: {
                let mut data: [[[MaybeUninit<Box<dyn Block>>; 16]; 16]; 16] =
                    unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..16 {
                    for j in 0..16 {
                        for k in 0..16 {
                            let x: i32 = px + i;
                            let y: i32 = py + k;
                            let z: i32 = pz + j;
                            let level: i32 =
                                ((x as f32 / 3.0 + (z as f32 / 5.0).sin()).sin() * 4.0) as i32;
                            if y < level {
                                if y >= 0 {
                                    data[i as usize][k as usize][j as usize].write(Box::new(Grass));
                                } else {
                                    data[i as usize][k as usize][j as usize].write(Box::new(Stone));
                                }
                            } else {
                                data[i as usize][k as usize][j as usize].write(Box::new(Air));
                            }
                        }
                    }
                }
                unsafe { transmute::<_, [[[Box<dyn Block>; 16]; 16]; 16]>(data) }
            },
        }
    }

    /// Get a block, None if outside of chunk
    pub fn get_block(&self, pos: BlockPos) -> Option<&Box<dyn Block>> {
        if pos.is_in_chunk() {
            Some(&self.blocks[pos.x as usize][pos.y as usize][pos.z as usize])
        } else {
            None
        }
    }

    /// Set a block, panics if outside a chunk
    pub fn set_block<T: 'static + Block + Clone>(&mut self, pos: BlockPos, b: &T) {
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = Box::new(b.clone());
    }

    /// Render if model present
    pub fn render(&self) {
        if let Some(x) = &self.model {
            x.render();
        }
    }

    /// Starts updating the chunk in another thread, responds by sending a ChunkUpdateInfo
    ///
    /// **Return Value**: The previous model (if present)
    pub fn start_update(sender: mpsc::Sender<ChunkUpdateInfo>, mut b: Box<Self>) -> Option<Model> {
        let model = b.model.take();
        thread::spawn(move || {
            let mesh = b.update();
            sender.send(ChunkUpdateInfo { mesh, chunk: b }).unwrap();
        });
        model
    }

    /// Updates the chunk (generates a mesh)
    ///
    /// Takes a couple of milliseconds
    fn update(&mut self) -> MeshData {
        let mut data = MeshData::new();
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    self.blocks[i][j][k].gen_mesh(
                        &mut data,
                        BlockPos::new(
                            self.pos.x * 16 + i as i32,
                            self.pos.y * 16 + j as i32,
                            self.pos.z * 16 + k as i32,
                        ),
                        block::BlockSides {
                            top: if j == 15 {
                                true
                            } else {
                                !self.blocks[i][j + 1][k].is_full()
                            },
                            bottom: if j == 0 {
                                true
                            } else {
                                !self.blocks[i][j - 1][k].is_full()
                            },
                            left: if i == 0 {
                                true
                            } else {
                                !self.blocks[i - 1][j][k].is_full()
                            },
                            right: if i == 15 {
                                true
                            } else {
                                !self.blocks[i + 1][j][k].is_full()
                            },
                            front: if k == 15 {
                                true
                            } else {
                                !self.blocks[i][j][k + 1].is_full()
                            },
                            back: if k == 0 {
                                true
                            } else {
                                !self.blocks[i][j][k - 1].is_full()
                            },
                        },
                    );
                }
            }
        }
        data
    }
}
