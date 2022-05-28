use super::*;

pub struct RaycastResult {
    pub point: Vec3,
    pub block: BlockPos,
    pub side: util::BlockSide,
}

pub fn raycast(
    server: &mut ChunkServer,
    block_manager: &BlockManager,
    mut point: Vec3,
    direction: Vec3,
) -> Option<RaycastResult> {
    let old_block = BlockPos::new(i32::MAX, i32::MAX, i32::MAX);
    let mut blocks_parsed = 0;
    loop {
        point += direction / 10.0;
        let block = BlockPos::new(
            point.x.floor() as _,
            point.y.floor() as _,
            point.z.floor() as _,
        );
        if block != old_block {
            if block_manager[server.get_block(block).unwrap()].solid {
                let center = Vec3::new(
                    block.x as f32 + 0.5,
                    block.y as f32 + 0.5,
                    block.z as f32 + 0.5,
                );
                let np = point - center;
                break Some(RaycastResult {
                    point,
                    block,
                    side: if np.x > np.y {
                        if np.x > -np.y {
                            util::BlockSide::Right
                        } else {
                            util::BlockSide::Bottom
                        }
                    } else {
                        if np.x > -np.y {
                            util::BlockSide::Top
                        } else {
                            util::BlockSide::Left
                        }
                    },
                });
            }
            blocks_parsed += 1;
            if blocks_parsed > 100 {
                break None;
            }
        }
    }
}
