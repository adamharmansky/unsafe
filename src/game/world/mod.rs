use super::*;
use block::Block;
use std::collections::HashMap;
use std::sync::mpsc;

mod chunk;
use chunk::Chunk;

enum StoredChunk {
    /// the chunk is being generated for the first time
    None,
    /// the chunk is being generated, showing previous model
    ///
    /// stored in a box so it can be transferred
    Model(Box<Model>),
    /// the chunk just exists, everything okay
    ///
    /// stored in a box so it can be transferred
    Chunk(Box<Chunk>),
}

pub struct ChunkUpdateInfo {
    mesh: MeshData,
    chunk: Box<Chunk>,
}

pub struct ChunkServer {
    chunks: HashMap<BlockPos, StoredChunk>,
    texture: Rc<Texture>,
    view_distance: i32,

    // Position of the camera (or anything else loading the chunks)
    pos: BlockPos,

    // for async chunk updates
    tx: mpsc::Sender<ChunkUpdateInfo>,
    rx: mpsc::Receiver<ChunkUpdateInfo>,
    /// number of chunks being generated
    pending: i32,
}

impl ChunkServer {
    pub fn new(texture: Rc<Texture>) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            chunks: HashMap::new(),
            texture,
            view_distance: 6,
            pos: BlockPos::new(std::i32::MAX, std::i32::MAX, std::i32::MAX),
            tx,
            rx,
            pending: 0,
        }
    }

    /// Load new and unload old chunks, can be safely called every frame
    pub fn update(&mut self, camera: BlockPos) {
        self.try_recv(64);
        let new_pos = BlockPos::new(camera.x / 16, camera.y / 16, camera.z / 16);
        if new_pos != self.pos {
            self.pos = new_pos;
            // remove old chunks
            self.chunks.retain(|k, _| {
                k.x >= self.pos.x - self.view_distance
                    && k.x < self.pos.x + self.view_distance
                    && k.y >= self.pos.y - self.view_distance
                    && k.y < self.pos.y + self.view_distance
                    && k.z >= self.pos.z - self.view_distance
                    && k.z < self.pos.z + self.view_distance
            });
            // insert new chunks
            for i in (self.pos.x - self.view_distance)..(self.pos.x + self.view_distance) {
                for j in (self.pos.y - self.view_distance)..(self.pos.y + self.view_distance) {
                    for k in (self.pos.z - self.view_distance)..(self.pos.z + self.view_distance) {
                        self.chunks
                            .entry(BlockPos::new(i, j, k))
                            .or_insert_with(|| {
                                Chunk::start_update(
                                    self.tx.clone(),
                                    Box::new(Chunk::new(BlockPos::new(i, j, k))),
                                );
                                self.pending += 1;
                                StoredChunk::None
                            });
                    }
                }
            }
        }
        if self.pending > 0 {
            println!("{} Pending chunks", self.pending);
        }
    }

    fn handle_received_chunk(&mut self, message: ChunkUpdateInfo) {
        match self.chunks.get_mut(&message.chunk.pos) {
            Some(x) => {
                if let StoredChunk::Chunk(_) = x {
                    // panic!("Attempted to insert into an already existing chunk!")
                    // Idk, do I care?
                    println!("Discarding already existing chunk!");
                }
                *x = StoredChunk::Chunk(message.chunk);
                match x {
                    StoredChunk::Chunk(x) => x.model = Some(Model::new(&message.mesh)),
                    _ => (),
                }
            }
            None => {
                println!("Discarding generated chunk");
            } // The chunk has been deleted during generation, discard it
        }
    }

    /// Receive at most `count` new chunks, should be called every frame
    fn try_recv(&mut self, count: i32) {
        for _ in 0..count {
            let message = self.rx.try_recv();
            let message = if let Ok(message) = message {
                message
            } else {
                break;
            };
            self.pending -= 1;
            assert!(self.pending >= 0);
            self.handle_received_chunk(message);
        }
    }

    fn sync(&mut self) {
        while self.pending > 0 {
            let message = self.rx.recv();
            let message = if let Ok(message) = message {
                message
            } else {
                break;
            };
            self.pending -= 1;
            assert!(self.pending >= 0);
            self.handle_received_chunk(message);
        }
    }

    /// Render everything, don't wait for chunks to generate
    pub fn render(&self) {
        self.texture.bind();
        for (_, v) in self.chunks.iter() {
            match v {
                StoredChunk::None => (),
                StoredChunk::Model(m) => {
                    m.render();
                }
                StoredChunk::Chunk(c) => {
                    c.render();
                }
            }
        }
    }

    #[allow(unused)]
    /// Get a block
    ///
    /// Panics if block is not there (will be fixed)
    pub fn get_block(&mut self, pos: BlockPos) -> &Box<dyn Block> {
        self.sync();
        match self
            .chunks
            .get(&BlockPos::new(pos.x / 16, pos.y / 16, pos.z / 16))
            .expect("Block doesn't exist!")
        {
            StoredChunk::Chunk(c) => c
                .get_block(BlockPos::new(pos.x % 16, pos.y % 16, pos.z % 16))
                .unwrap(),
            _ => panic!("Chunk doesn't exist!"),
        }
    }

    #[allow(unused)]
    /// Set a block
    ///
    /// Panics if block is not there (will be fixed)
    pub fn set_block<T: 'static + Block + Clone>(&mut self, pos: BlockPos, block: &T) {
        self.sync();
        match self
            .chunks
            .get_mut(&BlockPos::new(pos.x / 16, pos.y / 16, pos.z / 16))
            .expect("Block doesn't exist!")
        {
            StoredChunk::Chunk(c) => {
                c.set_block(BlockPos::new(pos.x % 16, pos.y % 16, pos.z % 16), block);
            }
            _ => panic!("Chunk doesn't exist!"),
        }
    }
}
