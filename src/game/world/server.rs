use std::collections::HashMap;
use std::sync::Arc;

use super::*;

enum ChunkGeneratorMessage {
    UpdateChunk(BlockPos),
    Exit,
}

struct ChunkUpdateInfo {
    mesh: MeshData,
    chunk: Box<Chunk>,
}

pub struct ChunkServer {
    chunks: HashMap<BlockPos, Option<Box<Chunk>>>,
    texture: Rc<Texture>,

    /// how far we can see in any direction
    ///
    /// One side od the box is view_distance*2+1
    view_distance: i32,

    // Position of the camera (or anything else loading the chunks)
    pos: BlockPos,

    generator_input: mpsc::Sender<ChunkGeneratorMessage>,
    generator_output: mpsc::Receiver<ChunkUpdateInfo>,

    pub block_manager: Arc<BlockManager>,
}

impl Drop for ChunkServer {
    fn drop(&mut self) {
        self.generator_input
            .send(ChunkGeneratorMessage::Exit)
            .unwrap();
        println!("Halting chunk generator thread!");
        self.recv();
    }
}

impl ChunkServer {
    /// Creates a new chunk server, initializes a secondary
    /// chunk generation thread.
    pub fn new(texture: Rc<Texture>, block_manager: Arc<BlockManager>) -> Self {
        let (our_generator_input, its_generator_input) = mpsc::channel::<ChunkGeneratorMessage>();
        let (its_generator_output, our_generator_output) = mpsc::channel::<ChunkUpdateInfo>();
        let clone = block_manager.clone();
        thread::spawn(move || {
            Self::run_generator(its_generator_input, its_generator_output, clone)
        });
        Self {
            chunks: HashMap::new(),
            texture,
            view_distance: 6,
            pos: BlockPos::new(std::i32::MAX, std::i32::MAX, std::i32::MAX),
            generator_input: our_generator_input,
            generator_output: our_generator_output,
            block_manager,
        }
    }

    fn run_generator(
        input: mpsc::Receiver<ChunkGeneratorMessage>,
        output: mpsc::Sender<ChunkUpdateInfo>,
        manager: Arc<BlockManager>,
    ) {
        loop {
            let msg = input.recv().unwrap();
            match msg {
                ChunkGeneratorMessage::UpdateChunk(p) => {
                    let mut chunk = Box::new(Chunk::new(p, &manager));
                    let mesh = chunk.update(&manager);
                    output.send(ChunkUpdateInfo { chunk, mesh }).unwrap();
                }
                ChunkGeneratorMessage::Exit => break,
            }
        }
    }

    fn request_generation(&self, pos: BlockPos) {
        self.generator_input
            .send(ChunkGeneratorMessage::UpdateChunk(pos))
            .unwrap();
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
                        let pos = BlockPos::new(i, j, k);
                        if !self.chunks.contains_key(&pos) {
                            self.request_generation(pos);
                            self.chunks.insert(pos, None);
                        }
                    }
                }
            }
        }
    }

    fn handle_received_chunk(&mut self, message: ChunkUpdateInfo) {
        match self.chunks.get_mut(&message.chunk.pos) {
            Some(x) => {
                if let Some(_) = x {
                    // panic!("Attempted to insert into an already existing chunk!")
                    // Idk, do I care?
                    println!("Discarding already existing chunk!");
                }
                *x = Some(message.chunk);
                match x {
                    Some(x) => x.model = Some(Model::new(&message.mesh)),
                    _ => (),
                }
            }
            None => {
                println!("Discarding generated chunk");
            } // The chunk has been deleted during generation, discard it
        }
    }

    /// Receive at most `count` new chunks, should be called every frame
    ///
    /// The limit is there to reduce lag spikes.
    fn try_recv(&mut self, count: i32) {
        for _ in 0..count {
            let message = self.generator_output.try_recv();
            let message = if let Ok(message) = message {
                message
            } else {
                break;
            };
            self.handle_received_chunk(message);
        }
    }

    /// Receive all chunks, useful when halting the generator
    ///
    /// Runs until there is an error
    fn recv(&mut self) {
        loop {
            let message = self.generator_output.recv();
            let message = if let Ok(message) = message {
                message
            } else {
                break;
            };
            self.handle_received_chunk(message);
        }
    }

    /// Render everything, don't wait for chunks to generate
    pub fn render(&self) {
        self.texture.bind();
        for (_, v) in self.chunks.iter() {
            if let Some(c) = v {
                c.render();
            }
        }
    }

    #[allow(unused)]
    /// Get a block
    pub fn get_block(&mut self, pos: BlockPos) -> Option<BlockID> {
        match self
            .chunks
            .get(&BlockPos::new(pos.x >> 4, pos.y >> 4, pos.z >> 4))
            .expect("Block doesn't exist!")
        {
            Some(c) => Some(
                c.get_block(BlockPos::new(pos.x & 15, pos.y & 15, pos.z & 15))
                    .unwrap(),
            ),
            None => None,
        }
    }

    #[allow(unused)]
    /// Set a block
    ///
    /// Panics if block is not there (will be fixed)
    pub fn set_block(&mut self, pos: BlockPos, block: BlockID) {
        match self
            .chunks
            .get_mut(&BlockPos::new(pos.x >> 4, pos.y >> 4, pos.z >> 4))
            .expect("Block doesn't exist!")
        {
            Some(c) => {
                c.set_block(BlockPos::new(pos.x & 15, pos.y & 15, pos.z & 15), block);
                c.model = Some(Model::new(&c.update(&self.block_manager)));
            }
            _ => todo!(),
        }
    }
}
