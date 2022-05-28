use super::*;

pub fn append_cube(
    data: &mut MeshData,
    pos: Vec3,
    sides: BlockSides,
    top_left: Vec2,
    bottom_right: Vec2,
) {
    let mut size: i32 = data.vertices.len() as _;
    if sides.back {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, -1.0));
        }
        size += 4;
    }
    if sides.left {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((-1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.front {
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, 1.0));
        }
        size += 4;
    }
    if sides.right {
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.top {
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((0.0, 1.0, 0.0));
        }
        size += 4;
    }
    if sides.bottom {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top_left.x, top_left.y));
        data.texcoords.push((top_left.x, bottom_right.y));
        data.texcoords.push((bottom_right.x, top_left.y));
        data.texcoords.push((bottom_right.x, bottom_right.y));
        for _ in 0..4 {
            data.normals.push((0.0, -1.0, 0.0));
        }
    }
}
