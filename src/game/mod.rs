mod block;
mod input;
mod player;
mod raycast;
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
            frag_normals = normalize(normals);
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
            light = light / 8.0 + 0.85;
            final_color = texture(textur, frag_texcoords) * light;
            final_color.w = 1.0;
        }"#;
    let shader = unsafe { Shader::new(VSCODE, FSCODE) };
    let view_matrix = shader.create_uniform("view");
    let model_matrix = shader.create_uniform("model");
    shader.bind();

    let mut game = Game::new();

    let mut input_state = InputState::new();

    let mut player = Player::new(Vec3::new(0.0, 10.0, 0.0));

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
                glClearColor(0.5, 0.8, 1.0, 1.0);
                glClear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                let mat = glam::Mat4::perspective_lh(1.0, 800.0 / 600.0, 0.1, 1000.0)
                    * glam::Mat4::from_rotation_x(-player.rotation.x)
                    * glam::Mat4::from_rotation_y(-player.rotation.y)
                    * glam::Mat4::from_translation(-player.pos);
                let model_mat = glam::Mat4::IDENTITY;
                shader.set_uniform(view_matrix, shader::Uniform::Mat4(mat));
                shader.set_uniform(model_matrix, shader::Uniform::Mat4(model_mat));
                game.chunks.render();
                context.swap_buffers().unwrap();
                input_state.reset();
            },
            _ => (),
        }
    });
}
