use glam::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[allow(unused)]
#[derive(Debug)]
pub struct BlockSides<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
    pub front: T,
    pub back: T,
}

#[allow(unused)]
#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum BlockSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockCollider {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
    pub d: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct TexRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl<T> From<std::collections::HashMap<BlockSide, T>> for BlockSides<Option<T>>
where
    T: Copy,
{
    fn from(x: std::collections::HashMap<BlockSide, T>) -> Self {
        Self {
            top: x.get(&BlockSide::Top).copied(),
            bottom: x.get(&BlockSide::Bottom).copied(),
            left: x.get(&BlockSide::Left).copied(),
            right: x.get(&BlockSide::Right).copied(),
            front: x.get(&BlockSide::Front).copied(),
            back: x.get(&BlockSide::Back).copied(),
        }
    }
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

impl std::ops::Add<BlockSide> for BlockPos {
    type Output = BlockPos;

    fn add(mut self, rhs: BlockSide) -> Self::Output {
        match rhs {
            BlockSide::Top => self.y += 1,
            BlockSide::Bottom => self.y -= 1,
            BlockSide::Left => self.x -= 1,
            BlockSide::Right => self.x += 1,
            BlockSide::Front => self.z += 1,
            BlockSide::Back => self.z -= 1,
        };
        self
    }
}

impl std::ops::AddAssign<BlockSide> for BlockPos {
    fn add_assign(&mut self, rhs: BlockSide) {
        *self = *self + rhs;
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

impl Into<glam::Vec3> for BlockPos {
    fn into(self) -> glam::Vec3 {
        glam::Vec3::new(self.x as _, self.y as _, self.z as _)
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
            BlockSide::Front => BlockPos::new(0, 0, 1),
            BlockSide::Back => BlockPos::new(0, 0, -1),
        }
    }
}

impl std::ops::Neg for BlockSide {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            BlockSide::Top => BlockSide::Bottom,
            BlockSide::Bottom => BlockSide::Top,
            BlockSide::Left => BlockSide::Right,
            BlockSide::Right => BlockSide::Left,
            BlockSide::Front => BlockSide::Back,
            BlockSide::Back => BlockSide::Front,
        }
    }
}
