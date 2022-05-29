use super::*;

pub struct RaycastResult {
    pub point: Vec3,
    pub block: BlockPos,
    pub side: util::BlockSide,
}

fn get_side(p: Vec3) -> util::BlockSide {
    println!("{:?}", p);
    let sides = [
        if p.x > p.y {
            if p.x > -p.y {
                util::BlockSide::Right
            } else {
                util::BlockSide::Bottom
            }
        } else {
            if p.x > -p.y {
                util::BlockSide::Top
            } else {
                util::BlockSide::Left
            }
        },
        if p.z > p.x {
            if p.z > -p.x {
                util::BlockSide::Front
            } else {
                util::BlockSide::Left
            }
        } else {
            if p.z > -p.x {
                util::BlockSide::Right
            } else {
                util::BlockSide::Back
            }
        },
        if p.y > p.z {
            if p.y > -p.z {
                util::BlockSide::Top
            } else {
                util::BlockSide::Back
            }
        } else {
            if p.y > -p.z {
                util::BlockSide::Front
            } else {
                util::BlockSide::Bottom
            }
        },
    ];
    println!("{:?}", sides);
    for i in 0..3 {
        for j in 0..i {
            println!("{}, {}", i, j);
            if sides[i] == sides[j] {
                return sides[i];
            }
        }
    }
    panic!("Failed raycast!");
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
                    side: get_side(np),
                });
            }
            blocks_parsed += 1;
            if blocks_parsed > 100 {
                break None;
            }
        }
    }
}
