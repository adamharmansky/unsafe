use super::*;

#[allow(unused)]
/// Describes the state of input, etc
pub struct InputState {
    /// Keys which are held down
    pub keys_down: std::collections::HashSet<glutin::event::VirtualKeyCode>,
    /// Keys which have been pressed in this frame
    pub keys_pressed: std::collections::HashSet<glutin::event::VirtualKeyCode>,
    /// Keys which have been released in this frame
    pub keys_released: std::collections::HashSet<glutin::event::VirtualKeyCode>,

    /// The five mouse buttons (Left, Middle, Right + Scroll wheel)
    pub pressed_buttons: [bool; 5],

    pub cursor: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys_down: std::collections::HashSet::new(),
            keys_pressed: std::collections::HashSet::new(),
            keys_released: std::collections::HashSet::new(),

            pressed_buttons: [false; 5],
            cursor: Vec2::new(0.0, 0.0),
        }
    }

    /// Return value: whether the window should close
    pub fn handle_input(
        &mut self,
        event: &glutin::event::WindowEvent,
        context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    ) -> bool {
        match event {
            glutin::event::WindowEvent::CloseRequested => {
                return true;
            }
            glutin::event::WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => {
                let code = input
                    .virtual_keycode
                    .unwrap_or_else(|| glutin::event::VirtualKeyCode::Space);
                match input.state {
                    glutin::event::ElementState::Pressed => {
                        self.keys_down.insert(code);
                        self.keys_pressed.insert(code);
                    }
                    glutin::event::ElementState::Released => {
                        self.keys_down.remove(&code);
                        self.keys_released.insert(code);
                    }
                }
            }
            glutin::event::WindowEvent::CursorMoved {
                device_id: _,
                position,
                ..
            } => {
                let size = context.window().inner_size();
                // if the cursor wasn't just returned
                if position.x != (size.width as i32 / 2) as f64
                    || position.y != (size.height as i32 / 2) as f64
                {
                    self.cursor.x = position.x as f32 - (size.width as i32 / 2) as f32;
                    self.cursor.y = position.y as f32 - (size.height as i32 / 2) as f32;
                    context
                        .window()
                        .set_cursor_position(glutin::dpi::PhysicalPosition::new(
                            size.width / 2,
                            size.height / 2,
                        ))
                        .unwrap();
                }
            }
            _ => (),
        }
        false
    }

    /// Call this function at the end of every frame
    pub fn reset(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.cursor = Vec2::new(0.0, 0.0);
    }
}
