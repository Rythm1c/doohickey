extern crate gl;
pub struct Shadow {
    pub depth_fbo: u32,
    pub texture: u32,
}

impl Shadow {
    pub fn new(w: i32, h: i32) -> Shadow {
        let mut buffer = 0;
        let mut tex = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut buffer);
            gl::GenTextures(1, &mut tex);

            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as i32,
                w,
                h,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            let border: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, &border[0]);

            gl::BindFramebuffer(gl::FRAMEBUFFER, buffer);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                tex,
                0,
            );
            gl::DrawBuffer(gl::NONE);
            gl::ReadBuffer(gl::NONE);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Shadow {
            depth_fbo: buffer,
            texture: tex,
        }
    }
    /// attach for rendering
    pub fn attach(&self, w: i32, h: i32) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.depth_fbo);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, w, h);
        }
    }
    /// back to default frame buffer
    pub fn detach() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}

impl Drop for Shadow {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture);
            gl::DeleteFramebuffers(1, &mut self.depth_fbo);
        }
    }
}
