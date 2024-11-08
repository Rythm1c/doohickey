/* extern crate gl;
use crate::math::vec3::*;

use crate::src::model;

pub struct Framebuffer {
    fbo: u32,
    rbo: u32,
    pub frame: u32,
    pub quad: model::Model,
}

impl Framebuffer {
    pub fn new() -> Result<Framebuffer, String> {
        let mut q = model::Model::new(
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        model::load_quad(&mut q);
        q.prepere_render_resources();
        q.textured = true;
        Ok(Framebuffer {
            fbo: 0,
            rbo: 0,
            frame: 0,
            quad: q,
        })
    }

    pub fn init(&mut self) {
        unsafe {
            gl::CreateFramebuffers(1, &mut self.fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            gl::GenTextures(1, &mut self.frame);
            gl::BindTexture(gl::TEXTURE_2D, self.frame);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                250,
                500,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.frame,
                0,
            );

            gl::GenRenderbuffers(1, &mut self.rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 250, 500);
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                self.rbo,
            );

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn attach(&self) {
        unsafe {
            gl::Viewport(0, 0, 250, 500);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }
    pub fn detach(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
    pub fn render(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.frame);
        }
        self.quad.render();
    }
}
impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.fbo);
            gl::DeleteRenderbuffers(1, &mut self.rbo);
            gl::DeleteTextures(1, &mut self.frame);
        }
    }
}
 */