use super::*;

pub struct Texture {
    id: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            glDeleteTextures(1, &self.id as _);
        }
    }
}

impl Texture {
    pub fn load(file: &str) -> Self {
        let img = image::io::Reader::open(file).unwrap().decode().unwrap();
        let mut id: u32 = 0;
        unsafe {
            glGenTextures(1, &mut id as _);
            glBindTexture(gl33::GL_TEXTURE_2D, id);
            glTexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_NEAREST.0 as _,
            );
            glTexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_NEAREST.0 as _,
            );
            glTexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                gl33::GL_RGBA.0 as _,
                img.width() as _,
                img.height() as _,
                0,
                match img {
                    image::DynamicImage::ImageLuma8(_) => gl33::GL_RED,
                    image::DynamicImage::ImageLumaA8(_) => gl33::GL_RG,
                    image::DynamicImage::ImageRgb8(_) => gl33::GL_RGB,
                    image::DynamicImage::ImageRgba8(_) => gl33::GL_RGBA,
                    _ => panic!("Wrong image format: {:?}", img),
                },
                gl33::GL_UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as _,
            );
            glGenerateMipmap(gl33::GL_TEXTURE_2D);
        }
        Texture { id }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(gl33::GL_TEXTURE_2D, self.id);
        }
    }

    #[allow(unused)]
    pub fn unbind(&self) {
        unsafe {
            glBindTexture(gl33::GL_TEXTURE_2D, 0);
        }
    }
}
