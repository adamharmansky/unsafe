use super::*;
use std::collections::HashMap;

mod append_cube;

use util::BlockSides;

pub type BlockID = i32;

pub struct BlockType {
    pub gen_mesh: &'static (dyn Fn(&mut MeshData, BlockPos, BlockSides) + Sync),
    pub solid: bool,
    pub name: &'static str,
}

pub struct BlockManager {
    name_index: HashMap<String, BlockID>,
    blocks: Vec<BlockType>,
}

unsafe impl Sync for BlockType {}
unsafe impl Sync for BlockManager {}

impl BlockManager {
    pub fn new() -> Self {
        let mut blocks = BlockManager {
            name_index: HashMap::new(),
            blocks: Vec::new(),
        };
        blocks.add_block(BlockType {
            gen_mesh: &|_, _, _| {},
            solid: false,
            name: "air",
        });
        blocks.add_block(BlockType {
            gen_mesh: &|data, pos, sides| {
                append_cube::append_cube(
                    data,
                    Vec3::new(pos.x as _, pos.y as _, pos.z as _),
                    sides,
                    Vec2::new(16.0 / 1024.0, 0.0),
                    Vec2::new(32.0 / 1024.0, 16.0 / 1024.0),
                )
            },
            solid: true,
            name: "stone",
        });
        blocks.add_block(BlockType {
            gen_mesh: &|data, pos, sides| {
                append_cube::append_cube(
                    data,
                    Vec3::new(pos.x as _, pos.y as _, pos.z as _),
                    sides,
                    Vec2::new(32.0 / 1024.0, 0.0),
                    Vec2::new(48.0 / 1024.0, 16.0 / 1024.0),
                )
            },
            solid: true,
            name: "grass",
        });
        blocks.add_block(BlockType {
            gen_mesh: &|data, pos, sides| {
                append_cube::append_cube(
                    data,
                    Vec3::new(pos.x as _, pos.y as _, pos.z as _),
                    sides,
                    Vec2::new(48.0 / 1024.0, 0.0),
                    Vec2::new(64.0 / 1024.0, 16.0 / 1024.0),
                )
            },
            solid: true,
            name: "planks",
        });
        blocks.add_block(BlockType {
            gen_mesh: &|data, pos, sides| {
                append_cube::append_cube(
                    data,
                    Vec3::new(pos.x as _, pos.y as _, pos.z as _),
                    sides,
                    Vec2::new(0.0, 16.0 / 1024.0),
                    Vec2::new(128.0 / 1024.0, 144.0 / 1024.0),
                )
            },
            solid: true,
            name: "harold",
        });
        blocks
    }

    pub fn add_block(&mut self, t: BlockType) -> BlockID {
        self.name_index
            .insert(String::from(t.name), self.blocks.len() as _);
        self.blocks.push(t);
        self.blocks.len() as i32
    }
}

impl std::ops::Index<BlockID> for BlockManager {
    type Output = BlockType;

    fn index(&self, index: BlockID) -> &Self::Output {
        &self.blocks[index as usize]
    }
}

impl std::ops::Index<String> for BlockManager {
    type Output = BlockID;

    fn index(&self, index: String) -> &Self::Output {
        &self.name_index[&index]
    }
}
