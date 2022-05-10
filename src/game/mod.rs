mod block;
mod blockpos;
mod world;

use crate::graphics::*;
use blockpos::BlockPos;
use std::rc::Rc;
use world::*;

pub fn start() {
    let evloop = glutin::event_loop::EventLoop::new();
    let builder = glutin::window::WindowBuilder::new()
        .with_title("Bogos binted")
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_resizable(false);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(builder, &evloop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    load_gl(&context);
    unsafe {
        glEnable(gl33::GL_TEXTURE_2D);
        glEnable(gl33::GL_DEPTH_TEST);
        glEnable(gl33::GL_BLEND);
        glBlendFunc(gl33::GL_SRC_ALPHA, gl33::GL_ONE_MINUS_SRC_ALPHA);
    }

    const VSCODE: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec2 texCoords;
        layout (location = 2) in vec3 normals;
        uniform mat4 view;
        uniform mat4 model;
        out vec2 frag_texcoords;
        out vec3 frag_normals;
        void main() {
            frag_texcoords = texCoords;
            frag_normals = normals;
            gl_Position = view * model * vec4(pos, 1.0);
        }"#;
    const FSCODE: &str = r#"#version 330 core
        out vec4 final_color;
        in vec2 frag_texcoords;
        in vec3 frag_normals;
        uniform sampler2D textur;
        void main() {
            // final_color = texture(textur, frag_texcoords);
            float light = dot(frag_normals, normalize(vec3(1.0, 2.0, -1.0)));
            light = light / 2.0 + 0.5;
            final_color = texture(textur, frag_texcoords) * light;
            final_color.w = 1.0;
        }"#;
    let shader = unsafe { Shader::new(VSCODE, FSCODE) };
    let view_matrix = shader.create_uniform("view");
    let model_matrix = shader.create_uniform("model");
    shader.bind();

    let block_texture = Rc::new(Texture::load("blocks.png"));

    let mut chunks = ChunkServer::new(Rc::clone(&block_texture));
    chunks.update(BlockPos::new(0, 0, 0));

    let mut player_pos = Vec3::new(0.0, 10.0, 0.0);
    let mut player_rotation = Vec2::new(0.0, 0.0);

    let mut pressed_keys = std::collections::HashSet::<glutin::event::VirtualKeyCode>::new();

    evloop.run(move |ev, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Wait;

        match ev {
            glutin::event::Event::LoopDestroyed => return,
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
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
                            pressed_keys.insert(code);
                        }
                        glutin::event::ElementState::Released => {
                            pressed_keys.remove(&code);
                        }
                    }
                }
                glutin::event::WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    ..
                } => {
                    let size = context.window().inner_size();
                    let pos = (
                        position.x as i32 - (size.width / 2) as i32,
                        position.y as i32 - (size.height / 2) as i32,
                    );
                    player_rotation.x += pos.1 as f32 / 100.0;
                    player_rotation.y += pos.0 as f32 / 100.0;
                    context
                        .window()
                        .set_cursor_position(glutin::dpi::PhysicalPosition::new(
                            size.width / 2,
                            size.height / 2,
                        ))
                        .unwrap();
                }
                _ => (),
            },
            glutin::event::Event::MainEventsCleared => unsafe {
                let front = glam::Mat4::from_rotation_y(player_rotation.y)
                    * glam::Mat4::from_rotation_x(player_rotation.x);
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::W) {
                    player_pos += front.transform_point3(Vec3::new(0.0, 0.0, 0.2));
                }
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::S) {
                    player_pos -= front.transform_point3(Vec3::new(0.0, 0.0, 0.2));
                }
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::A) {
                    player_pos -= front.transform_point3(Vec3::new(0.2, 0.0, 0.0));
                }
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::D) {
                    player_pos += front.transform_point3(Vec3::new(0.2, 0.0, 0.0));
                }
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::Space) {
                    player_pos.y += 0.2;
                }
                if pressed_keys.contains(&glutin::event::VirtualKeyCode::LControl) {
                    player_pos.y -= 0.2;
                }
                chunks.update(BlockPos::new(
                    player_pos.x as _,
                    player_pos.y as _,
                    player_pos.z as _,
                ));
                glClearColor(0.5, 0.8, 1.0, 1.0);
                glClear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                let mat = glam::Mat4::perspective_lh(1.0, 800.0 / 600.0, 0.1, 1000.0)
                    * glam::Mat4::from_rotation_x(-player_rotation.x)
                    * glam::Mat4::from_rotation_y(-player_rotation.y)
                    * glam::Mat4::from_translation(-player_pos);
                let model_mat = glam::Mat4::IDENTITY;
                shader.set_uniform(view_matrix, shader::Uniform::Mat4(mat));
                shader.set_uniform(model_matrix, shader::Uniform::Mat4(model_mat));
                chunks.render();
                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
