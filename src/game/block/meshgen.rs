use super::*;
use util::TexRect;

pub fn append_cube(data: &mut MeshData, pos: Vec3, sides: BlockSides<bool>, texture: TexRect) {
    let mut size: i32 = data.vertices.len() as _;
    if sides.back {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.right, texture.bottom));
        data.texcoords.push((texture.left, texture.top));
        data.texcoords.push((texture.right, texture.top));
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
        data.texcoords.push((texture.right, texture.bottom));
        data.texcoords.push((texture.right, texture.top));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.left, texture.top));
        for _ in 0..4 {
            data.normals.push((-1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.front {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
        // Debug message
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((texture.right, texture.bottom));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.right, texture.top));
        data.texcoords.push((texture.left, texture.top));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, 1.0));
        }
        size += 4;
    }
    if sides.right {
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.left, texture.top));
        data.texcoords.push((texture.right, texture.bottom));
        data.texcoords.push((texture.right, texture.top));
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
        data.texcoords.push((texture.left, texture.top));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.right, texture.top));
        data.texcoords.push((texture.right, texture.bottom));
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
        data.texcoords.push((texture.left, texture.top));
        data.texcoords.push((texture.left, texture.bottom));
        data.texcoords.push((texture.right, texture.top));
        data.texcoords.push((texture.right, texture.bottom));
        for _ in 0..4 {
            data.normals.push((0.0, -1.0, 0.0));
        }
    }
}

pub fn append_cube_sided(
    data: &mut MeshData,
    pos: Vec3,
    sides: BlockSides<bool>,
    side: TexRect,
    top: TexRect,
    bottom: TexRect,
) {
    let mut size: i32 = data.vertices.len() as _;
    if sides.back {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.left, side.top));
        data.texcoords.push((side.right, side.top));
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
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.right, side.top));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.left, side.top));
        for _ in 0..4 {
            data.normals.push((-1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.front {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 1.0, pos.z + 1.0));
        // Debug message
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.right, side.top));
        data.texcoords.push((side.left, side.top));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, 1.0));
        }
        size += 4;
    }
    if sides.right {
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 1.0, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.left, side.top));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.right, side.top));
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
        data.texcoords.push((top.left, top.top));
        data.texcoords.push((top.left, top.bottom));
        data.texcoords.push((top.right, top.top));
        data.texcoords.push((top.right, top.bottom));
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
        data.texcoords.push((bottom.left, bottom.top));
        data.texcoords.push((bottom.left, bottom.bottom));
        data.texcoords.push((bottom.right, bottom.top));
        data.texcoords.push((bottom.right, bottom.bottom));
        for _ in 0..4 {
            data.normals.push((0.0, -1.0, 0.0));
        }
    }
}

pub fn append_slab(
    data: &mut MeshData,
    pos: Vec3,
    sides: BlockSides<bool>,
    side: TexRect,
    top: TexRect,
    bottom: TexRect,
) {
    let mut size: i32 = data.vertices.len() as _;
    if sides.back {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 0.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.left, side.top));
        data.texcoords.push((side.right, side.top));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, -1.0));
        }
        size += 4;
    }
    if sides.left {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.right, side.top));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.left, side.top));
        for _ in 0..4 {
            data.normals.push((-1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.front {
        data.vertices.push((pos.x + 0.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 1.0));
        // Debug message
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.right, side.top));
        data.texcoords.push((side.left, side.top));
        for _ in 0..4 {
            data.normals.push((0.0, 0.0, 1.0));
        }
        size += 4;
    }
    if sides.right {
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.0, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((side.left, side.bottom));
        data.texcoords.push((side.left, side.top));
        data.texcoords.push((side.right, side.bottom));
        data.texcoords.push((side.right, side.top));
        for _ in 0..4 {
            data.normals.push((1.0, 0.0, 0.0));
        }
        size += 4;
    }
    if sides.top {
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 0.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 0.0));
        data.vertices.push((pos.x + 0.0, pos.y + 0.5, pos.z + 1.0));
        data.vertices.push((pos.x + 1.0, pos.y + 0.5, pos.z + 1.0));
        data.indices.push((0 + size, 1 + size, 2 + size));
        data.indices.push((1 + size, 2 + size, 3 + size));
        data.texcoords.push((top.left, top.top));
        data.texcoords.push((top.left, top.bottom));
        data.texcoords.push((top.right, top.top));
        data.texcoords.push((top.right, top.bottom));
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
        data.texcoords.push((bottom.left, bottom.top));
        data.texcoords.push((bottom.left, bottom.bottom));
        data.texcoords.push((bottom.right, bottom.top));
        data.texcoords.push((bottom.right, bottom.bottom));
        for _ in 0..4 {
            data.normals.push((0.0, -1.0, 0.0));
        }
    }
}
