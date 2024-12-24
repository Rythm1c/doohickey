use super::buffer::*;
use super::material::*;
use super::texture::Texture;
use super::vao::Vao;

#[derive(Clone)]
pub struct Mesh {
    pub vao: Vao,
    pub vbo: VBO,
    pub ebo: Option<EBO>,
    pub texture: Option<Texture>,
    pub material: Material,
}

impl Mesh {
    pub fn default() -> Self {
        Self {
            vao: Vao::new(),
            vbo: VBO::default(),
            ebo: None,
            texture: None,
            material: Material::BlinnPhong(Phong::default()),
        }
    }

    pub fn create(&mut self) {
        self.vao.create();
        self.vbo.create(gl::ARRAY_BUFFER);
        if let Some(ebo) = &mut self.ebo {
            ebo.create(gl::ELEMENT_ARRAY_BUFFER);
        }
        Vao::set_attributes();
        Vao::unbind();
    }

    pub fn textured(&self) -> bool {
        match self.texture {
            Some(..) => true,

            None => false,
        }
    }

    pub fn render(&mut self) {
        // use gl::DrawElements if mesh contains an index buffer

        if let Some(ebo) = &mut self.ebo {
            //bind texture only if mesh contains one
            if let Some(texture) = &self.texture {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE1);
                }
                texture.bind();
            }

            unsafe {
                self.vao.bind();
                gl::DrawElements(
                    gl::TRIANGLES,
                    ebo.data.len().try_into().unwrap(),
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                Vao::unbind();
            }
        }
        // else use gl::DrawArrays for none indexed drawing
        else {
            unsafe {
                self.vao.bind();
                gl::DrawArrays(gl::TRIANGLES, 0, self.vbo.data.len() as i32);
                Vao::unbind();
            }
        }
    }
}
