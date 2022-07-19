mod block;
mod input;
mod player;
mod raycast;
mod render_view;
mod shaders;
mod util;
mod world;

use crate::graphics::*;
use block::BlockManager;
use input::InputState;
use player::Player;
use render_view::RenderView;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use util::BlockPos;
use world::*;

pub struct Game {
    pub input: InputState,
    pub blocks: Arc<BlockManager>,
    pub chunks: ChunkServer,
    pub config: json::JsonValue,
}

impl Game {
    pub const GRAVITY: f32 = -0.01;
    pub fn new() -> Self {
        let config = {
            let mut config_file = File::open("config.json").expect("cannot open config file");
            let mut config_json = String::new();
            config_file.read_to_string(&mut config_json).unwrap();
            json::parse(config_json.as_str()).unwrap()
        };
        let manager = Arc::new(BlockManager::new("blocks.json"));
        let clone = manager.clone();
        let chunks = ChunkServer::new(Rc::new(Texture::load("blocks.png")), clone, &config);

        let mut game = Game {
            input: InputState::new(),
            blocks: manager,
            chunks,
            config,
        };
        game.chunks.update(BlockPos::new(0, 0, 0));
        game
    }
}

pub fn start() {
    let evloop = glutin::event_loop::EventLoop::new();
    let builder = glutin::window::WindowBuilder::new()
        .with_title("Bogos binted")
        .with_inner_size(glutin::dpi::PhysicalSize::new(800, 600))
        .with_resizable(false);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(builder, &evloop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    // if let Err(e) = context.window().set_cursor_grab(true) {
    //     println!("Cannot grab cursor! The cursor will be visible in the center of the screen. Reason: {}", e);
    // }

    load_gl(&context);
    unsafe {
        glEnable(gl33::GL_TEXTURE_2D);
        glEnable(gl33::GL_DEPTH_TEST);
        glEnable(gl33::GL_BLEND);
        glBlendFunc(gl33::GL_SRC_ALPHA, gl33::GL_ONE_MINUS_SRC_ALPHA);
    }

    let mut game_view = RenderView::new(shaders::game_shader());
    let mut ui_view = RenderView::new(shaders::ui_shader());

    let mut game = Game::new();
    let mut player = Player::new(Vec3::new(0.0, 10.0, 0.0), &game);
    let mut input_state = InputState::new();

    let mut frames_since_message = 0;
    let mut last_time = std::time::Instant::now();

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

                glEnable(gl33::GL_DEPTH_TEST);
                game_view.bind();
                glClearColor(0.5, 0.8, 1.0, 1.0);
                glClear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                let aspect = {
                    let size = context.window().inner_size();
                    size.width as f32 / size.height as f32
                };
                let mat = glam::Mat4::perspective_lh(1.0, aspect, 0.1, 1000.0)
                    * glam::Mat4::from_rotation_x(-player.rotation.x)
                    * glam::Mat4::from_rotation_y(-player.rotation.y);
                game_view
                    .set_camera_position(player.pos + Vec3::new(0.0, Player::CAMERA_HEIGHT, 0.0));
                game_view.set_view(mat);
                game_view.set_model(glam::Mat4::IDENTITY);
                game.chunks.render();

                // now, render the UI
                glClear(gl33::GL_DEPTH_BUFFER_BIT);
                ui_view.bind();
                ui_view.set_view(glam::Mat4::from_scale(Vec3::new(1.0 / aspect, 1.0, 1.0)));
                ui_view.set_model(glam::Mat4::IDENTITY);
                player.draw_hotbar();
                context.swap_buffers().unwrap();
                input_state.reset();

                // Debug message
                if frames_since_message >= 60 {
                    let now = std::time::Instant::now();
                    println!(
                        "{:.2} FPS\t position: {} {} {}",
                        60000.0 / now.duration_since(last_time).as_millis() as f32,
                        player.pos.x,
                        player.pos.y,
                        player.pos.z
                    );
                    last_time = now;
                    frames_since_message = 0;
                }
                frames_since_message += 1;
            },
            _ => (),
        }
    });
}
