use super::texture::Texture;

#[derive(Clone)]
pub enum Material {
    BlinnPhong(Phong),
    Pbr(Pbr),
}

#[derive(Clone)]
pub struct Phong {
    pub base_color: [f32; 3],
    pub specular_factor: i32,
    pub diffuse_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
}

#[derive(Clone)]
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
            ao: 0.1,
            roughness: 0.5,
            base_color: [1.0; 3],
            metallic_factor: 0.5,
            base_texture: None,
            metallic_texture: None,
        }
    }
}

use super::shaders::Program;
impl Material {
    pub fn config_shader(&self, program: &mut Program) {
        match self {
            Material::BlinnPhong(phong) => {}
            Material::Pbr(pbr) => {}
        }
    }
}

/*

pub trait Materail {
    fn to_shader(&self, program: &mut Program);
}
 */
