use super::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use util::BlockCollider;

mod meshgen;

use util::BlockSides;

pub type BlockID = i32;

pub struct BlockType {
    pub gen_mesh: Arc<dyn Fn(&mut MeshData, BlockPos, BlockSides<bool>) + Sync + Send>,
    pub collider: Vec<BlockCollider>,
    pub solid: bool,
    pub name: String,
}

pub struct BlockManager {
    name_index: HashMap<String, BlockID>,
    blocks: Vec<BlockType>,
}

unsafe impl Sync for BlockType {}
unsafe impl Sync for BlockManager {}

impl BlockManager {
    /// Creates a new BlockManager loaded from a json file
    pub fn new(filename: &str) -> Self {
        let mut blocks = BlockManager {
            name_index: HashMap::new(),
            blocks: Vec::new(),
        };
        // load json
        let mut blocks_file =
            File::open(filename).expect(format!("cannot open file {}", filename).as_str());
        let mut blocks_json = String::new();
        blocks_file.read_to_string(&mut blocks_json).unwrap();
        let blocks_json = json::parse(blocks_json.as_str()).unwrap();
        // parse json
        for i in 0..blocks_json.len() {
            blocks.add_block(BlockType::from(&blocks_json[i]));
        }
        blocks
    }

    pub fn add_block(&mut self, t: BlockType) -> BlockID {
        self.name_index
            .insert(String::from(t.name.as_str()), self.blocks.len() as _);
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

impl From<&json::JsonValue> for BlockType {
    fn from(value: &json::JsonValue) -> Self {
        // mesh generation function
        let mesh_type = value["model"].as_str();
        let mesh_fn: Arc<dyn Fn(&mut MeshData, BlockPos, BlockSides<bool>) + Sync + Send> =
            match mesh_type {
                Some(mesh_type) => match mesh_type {
                    "block" => {
                        let texture = util::TexRect {
                            left: value["texture"]["left"].as_i32().unwrap() as f32 / 1024.0,
                            top: value["texture"]["top"].as_i32().unwrap() as f32 / 1024.0,
                            right: value["texture"]["right"].as_i32().unwrap() as f32 / 1024.0,
                            bottom: value["texture"]["bottom"].as_i32().unwrap() as f32 / 1024.0,
                        };
                        Arc::new(move |mesh, pos, sides| {
                            meshgen::append_cube(mesh, pos.into(), sides, texture)
                        })
                    }
                    "sided" => {
                        let top = util::TexRect {
                            left: value["textures"]["top"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["top"]["top"].as_i32().unwrap() as f32 / 1024.0,
                            right: value["textures"]["top"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["top"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        let bottom = util::TexRect {
                            left: value["textures"]["bottom"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["bottom"]["top"].as_i32().unwrap() as f32
                                / 1024.0,
                            right: value["textures"]["bottom"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["bottom"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        let side = util::TexRect {
                            left: value["textures"]["side"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["side"]["top"].as_i32().unwrap() as f32 / 1024.0,
                            right: value["textures"]["side"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["side"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        Arc::new(move |mesh, pos, sides| {
                            meshgen::append_cube_sided(mesh, pos.into(), sides, side, top, bottom)
                        })
                    }
                    "slab" => {
                        let top = util::TexRect {
                            left: value["textures"]["top"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["top"]["top"].as_i32().unwrap() as f32 / 1024.0,
                            right: value["textures"]["top"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["top"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        let bottom = util::TexRect {
                            left: value["textures"]["bottom"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["bottom"]["top"].as_i32().unwrap() as f32
                                / 1024.0,
                            right: value["textures"]["bottom"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["bottom"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        let side = util::TexRect {
                            left: value["textures"]["side"]["left"].as_i32().unwrap() as f32
                                / 1024.0,
                            top: value["textures"]["side"]["top"].as_i32().unwrap() as f32 / 1024.0,
                            right: value["textures"]["side"]["right"].as_i32().unwrap() as f32
                                / 1024.0,
                            bottom: value["textures"]["side"]["bottom"].as_i32().unwrap() as f32
                                / 1024.0,
                        };
                        Arc::new(move |mesh, pos, sides| {
                            meshgen::append_slab(mesh, pos.into(), sides, side, top, bottom)
                        })
                    }
                    _ => Arc::new(|_, _, _| {}),
                },
                None => Arc::new(|_, _, _| {}),
            };

        // load the collider
        let mut collider: Vec<BlockCollider> = Vec::new();
        let collider_json = &value["collider"];
        for i in 0..collider_json.len() {
            collider.push(BlockCollider {
                x: collider_json[i]["x"].as_f32().unwrap(),
                y: collider_json[i]["y"].as_f32().unwrap(),
                z: collider_json[i]["z"].as_f32().unwrap(),
                w: collider_json[i]["w"].as_f32().unwrap(),
                h: collider_json[i]["h"].as_f32().unwrap(),
                d: collider_json[i]["d"].as_f32().unwrap(),
            });
        }

        BlockType {
            gen_mesh: mesh_fn,
            collider,
            solid: value["solid"].as_bool().unwrap(),
            name: String::from(value["name"].as_str().unwrap()),
        }
    }
}
