mod block;
mod input;
mod player;
mod raycast;
mod shaders;
mod util;
mod world;

use crate::graphics::*;
use block::BlockManager;
use input::InputState;
use player::Player;
use std::rc::Rc;
use std::sync::Arc;
use util::BlockPos;
use world::*;

pub struct Game {
    pub input: InputState,
    pub blocks: Arc<BlockManager>,
    pub chunks: ChunkServer,
}

impl Game {
    pub fn new() -> Self {
        let manager = Arc::new(BlockManager::new());
        let clone = manager.clone();
        let mut game = Game {
            input: InputState::new(),
            blocks: manager,
            chunks: ChunkServer::new(Rc::new(Texture::load("blocks.png")), clone),
        };
        game.chunks.update(BlockPos::new(0, 0, 0));
        game
    }
}

pub fn start() {
    let evloop = glutin::event_loop::EventLoop::new();
    let builder = glutin::window::WindowBuilder::new()
        .with_title("Bogos binted")
        .with_inner_size(glutin::dpi::LogicalSize::new(800f32, 600f32))
        .with_resizable(false);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(builder, &evloop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    context.window().set_cursor_grab(true).unwrap();

    load_gl(&context);
    unsafe {
        glEnable(gl33::GL_TEXTURE_2D);
        glEnable(gl33::GL_DEPTH_TEST);
        glEnable(gl33::GL_BLEND);
        glBlendFunc(gl33::GL_SRC_ALPHA, gl33::GL_ONE_MINUS_SRC_ALPHA);
    }

    let game_shader = shaders::game_shader();
    let view_matrix = game_shader.create_uniform("view");
    let model_matrix = game_shader.create_uniform("model");

    let ui_shader = shaders::ui_shader();
    let ui_transform = ui_shader.create_uniform("view");

    let mut game = Game::new();

    let mut input_state = InputState::new();

    let mut player = Player::new(Vec3::new(0.0, 10.0, 0.0), &game);

    evloop.run(move |ev, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Wait;

        match ev {
            glutin::event::Event::LoopDestroyed => return,
            glutin::event::Event::WindowEvent { event, .. } => {
                if input_state.handle_input(&event, &context) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                if let glutin::event::WindowEvent::CursorMoved { device_id: _, .. } = event {}
            }
            glutin::event::Event::MainEventsCleared => unsafe {
                player.update(&input_state, &mut game);
                game.chunks.update(BlockPos::new(
                    player.pos.x as _,
                    player.pos.y as _,
                    player.pos.z as _,
                ));
                println!("{}", player.pos);
                let b = game.chunks.get_block(BlockPos::new(
                    player.pos.x.floor() as _,
                    player.pos.y.floor() as _,
                    player.pos.z.floor() as _,
                ));
                if let Some(x) = b {
                    println!("{}", game.blocks[x].name);
                }
                glEnable(gl33::GL_DEPTH_TEST);
                game_shader.bind();
                glClearColor(0.5, 0.8, 1.0, 1.0);
                glClear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                let aspect = {
                    let size = context.window().inner_size();
                    size.width as f32 / size.height as f32
                };
                let mat = glam::Mat4::perspective_lh(1.0, aspect, 0.1, 1000.0)
                    * glam::Mat4::from_rotation_x(-player.rotation.x)
                    * glam::Mat4::from_rotation_y(-player.rotation.y)
                    * glam::Mat4::from_translation(-player.pos);
                let model_mat = glam::Mat4::IDENTITY;
                game_shader.set_uniform(view_matrix, shader::Uniform::Mat4(mat));
                game_shader.set_uniform(model_matrix, shader::Uniform::Mat4(model_mat));
                game.chunks.render();

                // glDisable(gl33::GL_DEPTH_TEST);
                glClear(gl33::GL_DEPTH_BUFFER_BIT);
                ui_shader.bind();
                ui_shader.set_uniform(
                    ui_transform,
                    shader::Uniform::Mat4(glam::Mat4::from_scale(Vec3::new(
                        1.0 / aspect,
                        1.0,
                        1.0,
                    ))),
                );
                player.draw_hotbar();
                context.swap_buffers().unwrap();
                input_state.reset();
            },
            _ => (),
        }
    });
}
