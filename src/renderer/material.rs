use super::shaders::Program;
use super::texture::Texture;

use crate::src::math::vec3::Vec3;

#[derive(Clone)]
pub struct Phong {
    pub base_color: [f32; 3],
    pub specular_factor: f32,
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

impl Default for Phong {
    fn default() -> Self {
        Self {
            specular_factor: 32.0,
            base_color: [1.0; 3],
            diffuse_texture: None,
            specular_texture: None,
        }
    }
}

impl Phong {
    fn configure_shader(&self, program: &Program) {
        program.update_vec3("baseColor", Vec3::from(&self.base_color));
        program.update_float("specular_strength", self.specular_factor);
        program.update_int("hasDiffuseTex", self.diffuse_texture.is_some().into());
        program.update_int("hasSpecularTex", self.specular_texture.is_some().into());
    }
}

impl Default for Pbr {
    fn default() -> Self {
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
impl Pbr {
    fn configure_shader(&self, program: &Program) {
        program.update_vec3("baseColor", Vec3::from(&self.base_color));
        program.update_float("metallicFactor", self.metallic_factor);
        program.update_float("roughness", self.roughness);
        program.update_float("ao", self.ao);
        program.update_int("hasBaseTexture", self.metallic_texture.is_some().into());
        program.update_int("hasBaseTexture", self.metallic_texture.is_some().into());
    }
}

#[derive(Clone)]
pub enum Materail {
    Phong(Phong),
    Pbr(Pbr),
}

impl Materail {
    pub fn default() -> Self {
        Self::Phong(Phong::default())
    }

    pub fn configure_shader(&self, program: &Program) {
        match self {
            Self::Phong(phong) => {
                phong.configure_shader(program);
            }
            Self::Pbr(pbr) => {
                pbr.configure_shader(program);
            }
        }
    }
}
