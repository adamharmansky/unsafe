#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
