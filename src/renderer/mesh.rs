use super::ebo::Ebo;
use super::vao::Vao;

#[derive(Clone)]
pub struct Mesh {
    pub vao: Vao,
    pub ebo: Option<Ebo>,
}

impl Mesh {
    pub fn default() -> Self {
        Self {
            vao: Vao::new(),
            ebo: None,
        }
    }

    pub fn create(&mut self) {
        self.vao.create();
        if let Some(ebo) = &mut self.ebo {
            ebo.create();
        }
        Vao::unbind();
    }

    pub fn render(&mut self) {
        if let Some(ebo) = &mut self.ebo {
            unsafe {
                self.vao.bind();
                gl::DrawElements(
                    gl::TRIANGLES,
                    ebo.indices.len().try_into().unwrap(),
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                Vao::unbind();
            }
        } else {
            unsafe {
                self.vao.bind();
                gl::DrawArrays(gl::TRIANGLES, 0, self.vao.vertices.len() as i32);
                Vao::unbind();
            }
        }
    }
}
