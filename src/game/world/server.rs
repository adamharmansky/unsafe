use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

use super::*;

pub struct ChunkServer {
    chunks: HashMap<BlockPos, Box<Chunk>>,
    texture: Rc<Texture>,

    /// queue of the chunks to be generated, used so that we don't generate all the chunks at once
    gen_queue: VecDeque<BlockPos>,

    /// how far we can see in any direction
    ///
    /// One side od the box is view_distance*2+1
    view_distance: i32,

    // Position of the camera (or anything else loading the chunks)
    pos: BlockPos,

    pub block_manager: Arc<BlockManager>,
}

impl ChunkServer {
    pub fn new(texture: Rc<Texture>, block_manager: Arc<BlockManager>) -> Self {
        Self {
            chunks: HashMap::new(),
            texture,
            view_distance: 12,
            pos: BlockPos::new(std::i32::MAX, std::i32::MAX, std::i32::MAX),
            block_manager,
            gen_queue: VecDeque::new(),
        }
    }

    /// Load new and unload old chunks, can be safely called every frame
    pub fn update(&mut self, camera: BlockPos) {
        self.handle_generation(16);
        let new_pos = BlockPos::new(camera.x >> 4, camera.y >> 4, camera.z >> 4);
        if new_pos != self.pos {
            self.pos = new_pos;
            // remove old chunks
            // We keep chunks loaded even when they are 2 units away from the view distance
            self.chunks
                .retain(|k, _| Self::keep_chunk(self.pos, self.view_distance + 2, k));
            // insert new chunks
            for i in (self.pos.x - self.view_distance)..(self.pos.x + self.view_distance) {
                for j in (self.pos.y - self.view_distance)..(self.pos.y + self.view_distance) {
                    for k in (self.pos.z - self.view_distance)..(self.pos.z + self.view_distance) {
                        let pos = BlockPos::new(i, j, k);

                        if match self.chunks.get(&pos) {
                            Some(x) => x.model.is_none(),
                            None => true,
                        } && !self.gen_queue.contains(&pos)
                        {
                            self.request_generation(pos);
                        }
                    }
                }
            }
        }
    }

    // Add a chunk to the generation queue
    fn request_generation(&mut self, pos: BlockPos) {
        self.gen_queue.push_back(pos);
    }

    // Generate at most `count` chunks from the queue
    fn handle_generation(&mut self, count: i32) {
        let mut generated_chunks = 0;
        loop {
            if let Some(x) = self.gen_queue.pop_front() {
                // If the chunk is not needed anymore, don't generate it
                if !Self::keep_chunk(self.pos, self.view_distance, &x) {
                    continue;
                }

                let bm = self.block_manager.clone();
                let mut c = match self.chunks.remove(&x) {
                    Some(a) => a,
                    None => Box::new(Chunk::new(x, &self.block_manager)),
                };
                c.update(bm, self);
                self.chunks.insert(x, c);

                // Only add to the count when the generated funtion has actually been executed
                generated_chunks += 1;
            } else {
                break;
            }
            if generated_chunks >= count {
                break;
            }
        }
    }

    #[inline]
    fn keep_chunk(camera: BlockPos, view_distance: i32, k: &BlockPos) -> bool {
        k.x >= camera.x - view_distance
            && k.x < camera.x + view_distance
            && k.y >= camera.y - view_distance
            && k.y < camera.y + view_distance
            && k.z >= camera.z - view_distance
            && k.z < camera.z + view_distance
    }

    /// Render everything, don't wait for chunks to generate
    pub fn render(&self) {
        self.texture.bind();
        for (_, v) in self.chunks.iter() {
            v.render();
        }
    }

    /// Takes a chunk out of the hash map, updates it, and puts it back in
    fn update_chunk(&mut self, p: BlockPos) -> Option<()> {
        let mut c = match self.chunks.remove(&p) {
            Some(x) => x,
            None => {
                self.cache_chunk(p);
                self.chunks.remove(&p)?
            }
        };
        let bm = self.block_manager.clone();
        c.update(bm, self);
        self.chunks.insert(p, c);
        Some(())
    }

    /// Loads a chunk at a given posision without generating the model, if the chunk doesn't exist.
    ///
    /// The chunk will either get overwritten by a generated one or deleted later.
    fn cache_chunk(&mut self, pos: BlockPos) {
        self.chunks
            .insert(pos, Box::new(Chunk::new(pos, &self.block_manager)));
    }

    /// Get a block
    pub fn get_block(&mut self, pos: BlockPos) -> Option<BlockID> {
        let p = BlockPos::new(pos.x >> 4, pos.y >> 4, pos.z >> 4);
        match self.chunks.get(&p) {
            Some(x) => x,
            None => {
                self.cache_chunk(p);
                self.chunks.get(&p)?
            }
        }
        .get_block(BlockPos::new(pos.x & 15, pos.y & 15, pos.z & 15))
    }

    /// Set a block
    pub fn set_block(&mut self, pos: BlockPos, block: BlockID) -> Option<()> {
        let p = BlockPos::new(pos.x >> 4, pos.y >> 4, pos.z >> 4);
        let inner_pos = BlockPos::new(pos.x & 15, pos.y & 15, pos.z & 15);
        // we need to take the chunk out, update it, and put it back in 🤷.
        let mut c = match self.chunks.remove(&p) {
            Some(x) => x,
            None => {
                self.cache_chunk(p);
                self.chunks.remove(&p)?
            }
        };
        c.set_block(inner_pos, block);
        let bm = self.block_manager.clone();
        c.update(bm, self);
        self.chunks.insert(p, c);

        // Update the neighboring chunks, if it's needed
        if inner_pos.x == 0 {
            self.update_chunk(p + BlockPos::new(-1, 0, 0))?;
        } else if inner_pos.x == 15 {
            self.update_chunk(p + BlockPos::new(1, 0, 0))?;
        }
        if inner_pos.y == 0 {
            self.update_chunk(p + BlockPos::new(0, -1, 0))?;
        } else if inner_pos.y == 15 {
            self.update_chunk(p + BlockPos::new(0, 1, 0))?;
        }
        if inner_pos.z == 0 {
            self.update_chunk(p + BlockPos::new(0, 0, -1))?;
        } else if inner_pos.z == 15 {
            self.update_chunk(p + BlockPos::new(0, 0, 1))?;
        }

        Some(())
    }
}
