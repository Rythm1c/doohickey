use super::shaders::Program;
use super::texture::Texture;
pub struct Phong {
    pub base_color: [f32; 3],
    pub specular_factor: i32,
    pub diffuse_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
}

pub struct Pbr {
    pub ao: f32,
    pub base_color: [f32; 3],
    pub roughness: f32,
    pub metallic_factor: f32,
    pub base_texture: Option<Texture>,
    pub metallic_texture: Option<Texture>,
}

impl Phong {
    pub fn default() -> Self {
        Self {
            specular_factor: 0,
            base_color: [1.0; 3],
            diffuse_texture: None,
            specular_texture: None,
        }
    }
}

impl Pbr {
    pub fn default() -> Self {
        Self {
            ao: 0.0,
            roughness: 0.0,
            base_color: [1.0; 3],
            metallic_factor: 0.0,
            base_texture: None,
            metallic_texture: None,
        }
    }
}

pub trait Materail {
    fn to_shader(&self, program: &mut Program);
}
