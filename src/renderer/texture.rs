use gl;
use image;
use std::os::raw::c_void;

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn from(&mut self, path: &std::path::Path) -> Result<(), String> {
        let img = image::open(path).unwrap().flipv();

        let format = match img.color() {
            image::ColorType::Rgb8 => gl::RGB,
            image::ColorType::Rgba8 => gl::RGBA,

            _ => {
                return Err(String::from("image format not recognised!"));
            }
        };

        let pixels_buffer_ptr = {
            if format == gl::RGB {
                let pixels = img.as_rgb8().unwrap();
                pixels.as_ptr() as *mut c_void
            } else {
                let pixels = img.as_rgba8().unwrap();
                pixels.as_ptr() as *mut c_void
            }
        };

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
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                pixels_buffer_ptr,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        };

        Ok(())
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
