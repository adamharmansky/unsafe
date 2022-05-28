use glam::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[allow(unused)]
pub struct BlockSides {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

#[allow(unused)]
pub enum BlockSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl BlockPos {
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn is_in_chunk(&self) -> bool {
        self.x >= 0 && self.x < 16 && self.y >= 0 && self.y < 16 && self.z >= 0 && self.z < 16
    }
}

impl std::ops::Add<BlockPos> for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: BlockPos) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl BlockSide {
    #[inline]
    pub fn to_pos(&self) -> BlockPos {
        match self {
            BlockSide::Top => BlockPos::new(0, 1, 0),
            BlockSide::Bottom => BlockPos::new(0, -1, 0),
            BlockSide::Left => BlockPos::new(-1, 0, 0),
            BlockSide::Right => BlockPos::new(1, 0, 0),
            BlockSide::Front => BlockPos::new(1, 0, 0),
            BlockSide::Back => BlockPos::new(-1, 0, 0),
        }
    }
}
