use super::*;

pub struct BlockSides {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

impl MeshData {
    fn append_cube_simple(
        &mut self,
        pos: Vec3,
        sides: BlockSides,
        top_left: Vec2,
        bottom_right: Vec2,
    ) {
        let mut size: i32 = self.vertices.len() as _;
        if sides.back {
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
            size += 4;
        }
        if sides.left {
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
            size += 4;
        }
        if sides.front {
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
            size += 4;
        }
        if sides.right {
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
            size += 4;
        }
        if sides.top {
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
            self.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
            self.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
            size += 4;
        }
        if sides.bottom {
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
            self.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
            self.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
            self.indices.push((0 + size, 1 + size, 2 + size));
            self.indices.push((1 + size, 2 + size, 3 + size));
            self.texcoords.push((top_left.x, top_left.y));
            self.texcoords.push((top_left.x, bottom_right.y));
            self.texcoords.push((bottom_right.x, top_left.y));
            self.texcoords.push((bottom_right.x, bottom_right.y));
        }
    }
}

pub trait Block {
    fn is_full(&self) -> bool {
        true
    }
    fn gen_mesh(&self, data: &mut MeshData, pos: BlockPos, sides: BlockSides) {
        data.append_cube_simple(
            Vec3::new(pos.x as _, pos.y as _, pos.z as _),
            sides,
            Vec2::new(0.0, 0.0),
            Vec2::new(16.0 / 1024.0, 16.0 / 1024.0),
        );
    }
}

pub struct Air;
#[derive(Clone)]
pub struct Stone;
pub struct Grass;

impl Block for Air {
    fn is_full(&self) -> bool {
        false
    }
    fn gen_mesh(&self, _: &mut MeshData, _: BlockPos, _: BlockSides) {}
}

impl Block for Stone {
    fn gen_mesh(&self, data: &mut MeshData, pos: BlockPos, sides: BlockSides) {
        data.append_cube_simple(
            Vec3::new(pos.x as _, pos.y as _, pos.z as _),
            sides,
            Vec2::new(16.0 / 1024.0, 0.0 / 1024.0),
            Vec2::new(32.0 / 1024.0, 16.0 / 1024.0),
        );
    }
}

impl Block for Grass {
    fn gen_mesh(&self, data: &mut MeshData, pos: BlockPos, sides: BlockSides) {
        data.append_cube_simple(
            Vec3::new(pos.x as _, pos.y as _, pos.z as _),
            sides,
            Vec2::new(32.0 / 1024.0, 0.0 / 1024.0),
            Vec2::new(48.0 / 1024.0, 16.0 / 1024.0),
        );
    }
}
