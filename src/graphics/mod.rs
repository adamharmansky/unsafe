pub mod meshdata;
pub mod model;
pub mod shader;
pub mod texture;

pub use gl33::global_loader::*;
pub use meshdata::MeshData;
pub use model::Model;
pub use shader::Shader;
pub use texture::Texture;

pub use glam::*;

pub trait Renderable {
    fn render(&self);
}

pub fn load_gl(context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    unsafe {
        gl33::global_loader::load_global_gl(&|ptr| {
            let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
            let r_str = c_str.to_str().unwrap();
            context.get_proc_address(r_str)
        });
    }
}
