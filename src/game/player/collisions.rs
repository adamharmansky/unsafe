use super::*;
use util::{BlockCollider, BlockSide, BlockSides};

/// Tests whether two lines overlap on a 1D plane
fn overlap(aa: f32, ab: f32, ba: f32, bb: f32) -> bool {
    // I HATE FLOATING POINT NUMBERS I HATE FLOATING POINT NUMBERS I HATE FLOATING POINT NUMBERS I HATE FLOATING POINT NUMBERS
    const TOLERANCE: f32 = 0.02;
    (ab - TOLERANCE > ba && ab < bb - TOLERANCE)
        || (aa - TOLERANCE > ba && aa < bb - TOLERANCE)
        || (ba < ab - TOLERANCE && ba - TOLERANCE > aa)
        || (bb < ab - TOLERANCE && bb - TOLERANCE > aa)
}

fn collide(player: &Player, collider: BlockCollider) -> Option<(BlockSide, f32, f32)> {
    let colliding_x = overlap(
        player.pos.x - Player::RADIUS,
        player.pos.x + Player::RADIUS,
        collider.x,
        collider.x + collider.w,
    );
    let colliding_y = overlap(
        player.pos.y,
        player.pos.y + Player::HEIGHT,
        collider.y,
        collider.y + collider.h,
    );
    let colliding_z = overlap(
        player.pos.z - Player::RADIUS,
        player.pos.z + Player::RADIUS,
        collider.z,
        collider.z + collider.d,
    );

    // if colliding_x && colliding_y && colliding_z {
    //     panic!(
    //         "Colliding in all three dimensions: {:?}, {:?}",
    //         player.pos, collider
    //     );
    // }

    if colliding_y && colliding_z {
        Some(if player.pos.x > collider.x {
            (
                BlockSide::Right,
                player.pos.x - Player::RADIUS - collider.x - collider.w,
                collider.x + collider.w + Player::RADIUS,
            )
        } else {
            (
                BlockSide::Left,
                collider.x - player.pos.x - Player::RADIUS,
                collider.x - Player::RADIUS,
            )
        })
    } else if colliding_x && colliding_z {
        Some(if player.pos.y > collider.y {
            (
                BlockSide::Top,
                player.pos.y - collider.y - collider.h,
                collider.y + collider.h,
            )
        } else {
            (
                BlockSide::Bottom,
                collider.y - player.pos.y - Player::HEIGHT,
                collider.y - Player::HEIGHT,
            )
        })
    } else if colliding_x && colliding_y {
        Some(if player.pos.z > collider.z {
            (
                BlockSide::Front,
                player.pos.z - Player::RADIUS - collider.z - collider.d,
                collider.z + collider.d + Player::RADIUS,
            )
        } else {
            (
                BlockSide::Back,
                collider.z - player.pos.z - Player::RADIUS,
                collider.z - Player::RADIUS,
            )
        })
    } else {
        None
    }
}

pub fn check_collisions(player: &mut Player, game: &mut Game) -> BlockSides<Option<(f32, f32)>> {
    let player_pos = BlockPos::new(
        player.pos.x.floor() as _,
        player.pos.y.floor() as _,
        player.pos.z.floor() as _,
    );

    let mut distances = std::collections::HashMap::<BlockSide, (f32, f32)>::new();

    // get all colliders surrouding the player
    for i in (player_pos.x - 1)..(player_pos.x + 2) {
        for j in (player_pos.y - 1)..(player_pos.y + 4) {
            for k in (player_pos.z - 1)..(player_pos.z + 2) {
                let pos = BlockPos::new(i, j, k);
                if let Some(b) = game.chunks.get_block(pos) {
                    for l in &game.blocks[b].collider {
                        let result = collide(
                            player,
                            BlockCollider {
                                x: l.x + i as f32,
                                y: l.y + j as f32,
                                z: l.z + k as f32,
                                w: l.w,
                                h: l.h,
                                d: l.d,
                            },
                        );
                        if let Some(x) = result {
                            if let Some(y) = distances.get_mut(&x.0) {
                                if x.1 < y.0 {
                                    y.0 = x.1;
                                    y.1 = x.2;
                                }
                            } else {
                                distances.insert(x.0, (x.1, x.2));
                            }
                        }
                    }
                }
            }
        }
    }
    BlockSides::from(distances)
}

// Pretty much not needed aymore but I'll leave it here if I need to do more tests with collisions

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_collide() {
//         let cases = [
//             (
//                 Vec3::new(4.424844, 2.0, -4.2),
//                 BlockCollider {
//                     x: 4.0,
//                     y: 2.0,
//                     z: -4.0,
//                     w: 1.0,
//                     h: 1.0,
//                     d: 1.0,
//                 },
//             ),
//             (
//                 Vec3::new(0.2, 4.5, 9.2),
//                 BlockCollider {
//                     x: -1.0,
//                     y: 4.0,
//                     z: 8.0,
//                     w: 1.0,
//                     h: 1.0,
//                     d: 1.0,
//                 },
//             ),
//         ];
//         for i in cases {
//             let player = Player {
//                 pos: i.0,
//                 rotation: Vec2::new(0.0, 0.0),
//                 velocity: Vec3::new(0.0, 0.0, 0.0),
//                 selected_block: 0,
//                 hotbar: Vec::new(),
//                 item_models: Vec::new(),
//             };
//             collide(&player, i.1);
//         }
//     }
// }
