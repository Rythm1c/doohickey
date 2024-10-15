use gl;
use image;
use std::os::raw::c_void;

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn from(&mut self, path: &std::path::Path) {
        let img = image::open(path).unwrap().flipv();
        let pixels = img.as_rgb8().unwrap();
        let pixels_buffer_ptr = pixels.as_ptr() as *mut c_void;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.id);

            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels_buffer_ptr,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
