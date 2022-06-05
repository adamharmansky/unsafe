use super::*;

pub struct RaycastResult {
    pub point: Vec3,
    pub block: BlockPos,
    pub side: util::BlockSide,
}

fn get_ray_side(point: Vec3, direction: Vec3) -> (util::BlockSide, f32) {
    let candidates = [
        (util::BlockSide::Left, -point.x / direction.x),
        (util::BlockSide::Right, (1.0 - point.x) / direction.x),
        (util::BlockSide::Bottom, -point.y / direction.y),
        (util::BlockSide::Top, (1.0 - point.y) / direction.y),
        (util::BlockSide::Back, -point.z / direction.z),
        (util::BlockSide::Front, (1.0 - point.z) / direction.z),
    ];
    let mut smallest = (util::BlockSide::Bottom, std::f32::MAX);
    for i in candidates {
        if i.1 > 0.00 && i.1 < smallest.1 {
            println!("{}", i.1);
            smallest = i;
        }
    }
    smallest
}

pub fn raycast(
    server: &mut ChunkServer,
    block_manager: &BlockManager,
    mut point: Vec3,
    direction: Vec3,
) -> Option<RaycastResult> {
    let mut block = BlockPos::new(
        point.x.floor() as i32 - (point.x == point.x.floor() && direction.x < 0.0) as i32,
        point.y.floor() as i32 - (point.y == point.y.floor() && direction.y < 0.0) as i32,
        point.z.floor() as i32 - (point.z == point.z.floor() && direction.z < 0.0) as i32,
    );
    println!("performing raycast from block {:?}", block);
    for _ in 0..20 {
        // point += direction / 10.0;
        let side = get_ray_side(
            point - Vec3::new(block.x as _, block.y as _, block.z as _),
            direction,
        );
        point += direction * side.1;
        block += side.0;

        if block_manager[server.get_block(block)?].solid {
            return Some(RaycastResult {
                point,
                block,
                side: -side.0,
            });
        }
    }
    None
}
