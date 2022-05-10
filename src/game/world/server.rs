use std::collections::HashMap;

use super::*;

/// The state of a chunk in the chunk server
enum StoredChunk {
    /// the chunk is being generated for the first time
    None,
    /// the chunk is being generated, showing previous model
    ///
    /// stored in a box so it can be transferred
    Model(Model),
    /// the chunk just exists, everything okay
    ///
    /// stored in a box so it can be transferred
    Chunk(Box<Chunk>),
}

struct ChunkUpdateInfo {
    mesh: MeshData,
    chunk: Box<Chunk>,
}

pub struct ChunkServer {
    chunks: HashMap<BlockPos, StoredChunk>,
    texture: Rc<Texture>,

    /// how far we can see in any direction
    ///
    /// # Example
    ///
    /// View distance 3 looks like this:
    ///
    /// ```
    /// ###|###
    /// ###|###
    /// ###|###
    /// ---.---
    /// ###|###
    /// ###|###
    /// ###|###
    /// ```
    view_distance: i32,

    // Position of the camera (or anything else loading the chunks)
    pos: BlockPos,

    // The chunk pipeline:
    generator_input: mpsc::Sender<BlockPos>,
    updater_input: mpsc::Sender<Box<Chunk>>,
    updater_output: mpsc::Receiver<ChunkUpdateInfo>,

    generator: thread::JoinHandle<()>,
    updater: thread::JoinHandle<()>,
}

impl ChunkServer {
    /// Creates a new chunk server, initializes a secondary
    /// chunk generation thread.
    pub fn new(texture: Rc<Texture>) -> Self {
        let (our_generator_input, its_generator_input) = mpsc::channel::<BlockPos>();
        let (our_updater_input, its_updater_input) = mpsc::channel::<Box<Chunk>>();
        let (its_updater_output, our_updater_output) = mpsc::channel::<ChunkUpdateInfo>();
        Self {
            chunks: HashMap::new(),
            texture,
            view_distance: 6,
            pos: BlockPos::new(std::i32::MAX, std::i32::MAX, std::i32::MAX),
            generator_input: our_generator_input,
            updater_input: our_updater_input.clone(),
            updater_output: our_updater_output,
            generator: thread::spawn(move || {
                Self::run_generator(its_generator_input, our_updater_input)
            }),
            updater: thread::spawn(move || {
                Self::run_updater(its_updater_input, its_updater_output)
            }),
        }
    }

    fn run_updater(input: mpsc::Receiver<Box<Chunk>>, output: mpsc::Sender<ChunkUpdateInfo>) {
        loop {
            let mut msg = input.recv().unwrap();
            let mesh = msg.update();
            output.send(ChunkUpdateInfo { mesh, chunk: msg }).unwrap();
        }
    }

    fn run_generator(input: mpsc::Receiver<BlockPos>, output: mpsc::Sender<Box<Chunk>>) {
        loop {
            let msg = input.recv().unwrap();
            output.send(Box::new(Chunk::new(msg))).unwrap();
        }
    }

    fn request_generation(&self, pos: BlockPos) {
        self.generator_input.send(pos).unwrap();
    }

    fn request_update(&self, mut chunk: Box<Chunk>) -> Option<Model> {
        let model = chunk.model.take();
        self.updater_input.send(chunk).unwrap();
        model
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
                            self.chunks.insert(pos, StoredChunk::None);
                        }
                    }
                }
            }
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
    ///
    /// The limit is there to reduce lag spikes.
    fn try_recv(&mut self, count: i32) {
        for _ in 0..count {
            let message = self.updater_output.try_recv();
            let message = if let Ok(message) = message {
                message
            } else {
                break;
            };
            self.handle_received_chunk(message);
        }
    }

    /// Waits for all pending chunks to arrive
    fn sync(&mut self) {
        todo!();
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
        match self
            .chunks
            .get(&BlockPos::new(pos.x / 16, pos.y / 16, pos.z / 16))
            .expect("Block doesn't exist!")
        {
            StoredChunk::Chunk(c) => c
                .get_block(BlockPos::new(pos.x % 16, pos.y % 16, pos.z % 16))
                .unwrap(),
            _ => todo!(),
        }
    }

    #[allow(unused)]
    /// Set a block
    ///
    /// Panics if block is not there (will be fixed)
    pub fn set_block<T: 'static + Block + Clone>(&mut self, pos: BlockPos, block: &T) {
        match self
            .chunks
            .get_mut(&BlockPos::new(pos.x / 16, pos.y / 16, pos.z / 16))
            .expect("Block doesn't exist!")
        {
            StoredChunk::Chunk(c) => {
                c.set_block(BlockPos::new(pos.x % 16, pos.y % 16, pos.z % 16), block);
            }
            _ => todo!(),
        }
    }
}
