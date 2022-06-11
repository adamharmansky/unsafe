use crate::game::util::BlockSides;

use super::*;

use block::BlockID;

use glutin::event::VirtualKeyCode;

pub struct Player {
    pub pos: Vec3,
    pub rotation: Vec2,
    velocity: Vec3,

    selected_block: usize,

    hotbar: Vec<BlockID>,
    item_models: Vec<Model>,
}

impl Player {
    pub const HEIGHT: f32 = 1.8;
    pub const CAMERA_HEIGHT: f32 = 1.6;
    const SPEED: f32 = 0.08;
    const JUMP_AMOUNT: f32 = 0.25;
    pub fn new(pos: Vec3, game: &Game) -> Self {
        let hotbar = vec![
            game.blocks[String::from("stone")],
            game.blocks[String::from("grass")],
            game.blocks[String::from("planks")],
            game.blocks[String::from("harold")],
        ];
        let item_mat = Mat4::from_translation(Vec3::new(-0.8, 0.8, 0.0))
            * Mat4::from_scale(Vec3::new(0.15, 0.15, 0.15))
            * Mat4::from_translation(Vec3::new(0.0, -f32::sqrt(2.0) / 2.0, 0.0))
            * Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_4)
            * Mat4::from_rotation_y(-std::f32::consts::FRAC_PI_4);
        let item_models = hotbar
            .iter()
            .map(|x| {
                let mut data = MeshData::new();
                (game.blocks[*x as BlockID].gen_mesh)(
                    &mut data,
                    BlockPos::new(0, 0, 0),
                    BlockSides {
                        top: true,
                        bottom: true,
                        left: true,
                        right: true,
                        front: true,
                        back: true,
                    },
                );
                for i in &mut data.vertices {
                    let vec = item_mat.transform_point3(Vec3::new(i.0, i.1, i.2));
                    *i = (vec.x, vec.y, vec.z);
                }
                Model::new(&data)
            })
            .collect();
        Player {
            pos,
            rotation: Vec2::new(0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            selected_block: 3,
            hotbar,
            item_models,
        }
    }

    pub fn draw_hotbar(&mut self) {
        self.item_models[self.selected_block as usize].render();
    }

    fn check_collisions(&mut self, game: &mut Game) {
        for i in 0..2 {
            if let Some(x) = game.chunks.get_block(BlockPos::new(
                (self.pos.x + 0.2).floor() as _,
                self.pos.y.floor() as i32 + i,
                self.pos.z.floor() as _,
            )) {
                if game.blocks[x].solid {
                    self.pos.x = (self.pos.x + 0.2).floor() - 0.2;
                    self.velocity.x = 0.0;
                }
            }
            if let Some(x) = game.chunks.get_block(BlockPos::new(
                (self.pos.x - 0.2).floor() as _,
                self.pos.y.floor() as i32 + i,
                self.pos.z.floor() as _,
            )) {
                if game.blocks[x].solid {
                    self.pos.x = (self.pos.x - 0.2).ceil() + 0.2;
                    self.velocity.x = 0.0;
                }
            }
        }
        for i in 0..2 {
            if let Some(x) = game.chunks.get_block(BlockPos::new(
                self.pos.x.floor() as _,
                self.pos.y.floor() as i32 + i,
                (self.pos.z - 0.2).floor() as _,
            )) {
                if game.blocks[x].solid {
                    self.pos.z = (self.pos.z - 0.2).ceil() + 0.2;
                    self.velocity.z = 0.0;
                }
            }
            if let Some(x) = game.chunks.get_block(BlockPos::new(
                self.pos.x.floor() as _,
                self.pos.y.floor() as i32 + i,
                (self.pos.z + 0.2).floor() as _,
            )) {
                if game.blocks[x].solid {
                    self.pos.z = (self.pos.z + 0.2).floor() - 0.2;
                    self.velocity.z = 0.0;
                }
            }
        }
        if let Some(x) = game.chunks.get_block(BlockPos::new(
            self.pos.x.floor() as _,
            (self.pos.y + Self::HEIGHT).floor() as i32,
            self.pos.z.floor() as _,
        )) {
            if game.blocks[x].solid {
                self.pos.y = (self.pos.y + Self::HEIGHT).floor() - Self::HEIGHT;
                self.velocity.y = 0.0;
            }
        }
    }

    pub fn update(&mut self, input: &InputState, game: &mut Game) {
        let front = glam::Mat4::from_rotation_y(self.rotation.y)
            * glam::Mat4::from_rotation_x(self.rotation.x);
        let mut motion = Vec3::new(0.0, 0.0, 0.0);
        if input.keys_down.contains(&VirtualKeyCode::W) {
            motion += front.transform_point3(Vec3::new(0.0, 0.0, 1.0));
        }
        if input.keys_down.contains(&VirtualKeyCode::S) {
            motion -= front.transform_point3(Vec3::new(0.0, 0.0, 1.0));
        }
        if input.keys_down.contains(&VirtualKeyCode::A) {
            motion -= front.transform_point3(Vec3::new(1.0, 0.0, 0.0));
        }
        if input.keys_down.contains(&VirtualKeyCode::D) {
            motion += front.transform_point3(Vec3::new(1.0, 0.0, 0.0));
        }
        motion.y = 0.0;
        motion = motion.normalize_or_zero() * Self::SPEED;
        self.pos += motion;
        // if input.keys_down.contains(&VirtualKeyCode::LControl) {
        //     self.pos.y -= 0.2;
        // }

        let mut on_ground = false;

        self.velocity.y -= 0.02;

        if let Some(c) = raycast::raycast(
            &mut game.chunks,
            &game.blocks,
            self.pos + Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ) {
            let h = self.pos.y - c.point.y;
            if h <= -self.velocity.y {
                on_ground = true;
                self.pos.y = c.point.y;
                self.velocity.y = 0.0;
            }
        }
        if on_ground && self.velocity.y < 0.0 {
            self.pos.x += self.velocity.x;
            self.pos.z += self.velocity.z;
        } else {
            self.pos += self.velocity;
        };

        self.check_collisions(game);

        if on_ground && input.keys_down.contains(&VirtualKeyCode::Space) {
            self.velocity.y += Self::JUMP_AMOUNT;
        }

        self.rotation.x += input.cursor.y / 100.0;
        self.rotation.y += input.cursor.x / 100.0;

        if input.keys_pressed.contains(&VirtualKeyCode::Key1) {
            self.selected_block = 0;
        }
        if input.keys_pressed.contains(&VirtualKeyCode::Key2) {
            self.selected_block = 1;
        }
        if input.keys_pressed.contains(&VirtualKeyCode::Key3) {
            self.selected_block = 2;
        }
        if input.keys_pressed.contains(&VirtualKeyCode::Key4) {
            self.selected_block = 3;
        }

        if input.keys_pressed.contains(&VirtualKeyCode::E) {
            let pos = raycast::raycast(
                &mut game.chunks,
                &game.blocks,
                self.pos + Vec3::new(0.0, Self::CAMERA_HEIGHT, 0.0),
                front.transform_point3(Vec3::new(0.0, 0.0, 1.0)),
            );
            if let Some(res) = pos {
                game.chunks.set_block(
                    res.block + res.side.to_pos(),
                    self.hotbar[self.selected_block],
                );
            }
        }
        if input.keys_pressed.contains(&VirtualKeyCode::Q) {
            let pos = raycast::raycast(
                &mut game.chunks,
                &game.blocks,
                self.pos + Vec3::new(0.0, Self::CAMERA_HEIGHT, 0.0),
                front.transform_point3(Vec3::new(0.0, 0.0, 1.0)),
            );
            if let Some(res) = pos {
                game.chunks
                    .set_block(res.block, game.blocks[String::from("air")]);
            }
        }
    }
}
