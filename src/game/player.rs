use super::*;

use glutin::event::VirtualKeyCode;

pub struct Player {
    pub pos: Vec3,
    pub rotation: Vec2,
}

impl Player {
    pub fn new(pos: Vec3) -> Self {
        Player {
            pos,
            rotation: Vec2::new(0.0, 0.0),
        }
    }
    pub fn update(&mut self, input: &InputState, game: &mut Game) {
        let front = glam::Mat4::from_rotation_y(self.rotation.y)
            * glam::Mat4::from_rotation_x(self.rotation.x);
        if input.keys_down.contains(&VirtualKeyCode::W) {
            self.pos += front.transform_point3(Vec3::new(0.0, 0.0, 0.2));
        }
        if input.keys_down.contains(&VirtualKeyCode::S) {
            self.pos -= front.transform_point3(Vec3::new(0.0, 0.0, 0.2));
        }
        if input.keys_down.contains(&VirtualKeyCode::A) {
            self.pos -= front.transform_point3(Vec3::new(0.2, 0.0, 0.0));
        }
        if input.keys_down.contains(&VirtualKeyCode::D) {
            self.pos += front.transform_point3(Vec3::new(0.2, 0.0, 0.0));
        }
        if input.keys_down.contains(&VirtualKeyCode::Space) {
            self.pos.y += 0.2;
        }
        if input.keys_down.contains(&VirtualKeyCode::LControl) {
            self.pos.y -= 0.2;
        }
        self.rotation.x += input.cursor.y / 100.0;
        self.rotation.y += input.cursor.x / 100.0;

        if input.keys_pressed.contains(&VirtualKeyCode::E) {
            let pos = raycast::raycast(
                &mut game.chunks,
                &game.blocks,
                self.pos,
                front.transform_point3(Vec3::new(0.0, 0.0, 1.0)),
            );
            if let Some(res) = pos {
                game.chunks.set_block(
                    res.block + res.side.to_pos(),
                    game.blocks[String::from("harold")],
                );
            }
        }
        if input.keys_pressed.contains(&VirtualKeyCode::Q) {
            let pos = raycast::raycast(
                &mut game.chunks,
                &game.blocks,
                self.pos,
                front.transform_point3(Vec3::new(0.0, 0.0, 1.0)),
            );
            if let Some(res) = pos {
                game.chunks
                    .set_block(res.block, game.blocks[String::from("air")]);
            }
        }
    }
}
