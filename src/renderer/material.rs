use super::shaders::Program;
use super::texture::Texture;

use crate::src::math::vec3::Vec3;

pub trait Materail {
    fn configure_shader(&self, program: &Program);
}

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

impl Materail for Phong {
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
impl Materail for Pbr {
    fn configure_shader(&self, program: &Program) {
        program.update_vec3("baseColor", Vec3::from(&self.base_color));
        program.update_float("metallicFactor", self.metallic_factor);
        program.update_float("roughness", self.roughness);
        program.update_float("ao", self.ao);
        program.update_int("hasBaseTexture", self.metallic_texture.is_some().into());
        program.update_int("hasBaseTexture", self.metallic_texture.is_some().into());
    }
}

/* #[derive(Clone)]
pub enum Material {
    BlinnPhong(Phong),
    Pbr(Pbr),
}
 */

/* impl Material {
    pub fn configure_shader(&self, program: &Program) {
        match self {
            Material::BlinnPhong(phong) => {
                program.update_vec3("baseColor", Vec3::from(&phong.base_color));
                program.update_float("specular_strength", phong.specular_factor);
                program.update_int("hasDiffuseTex", phong.diffuse_texture.is_some().into());
                program.update_int("hasSpecularTex", phong.specular_texture.is_some().into());
            }
            Material::Pbr(pbr) => {
                program.update_vec3("baseColor", Vec3::from(&pbr.base_color));
                program.update_float("metallicFactor", pbr.metallic_factor);
                program.update_float("roughness", pbr.roughness);
                program.update_float("ao", pbr.ao);
                program.update_int("hasBaseTexture", pbr.metallic_texture.is_some().into());
                program.update_int("hasBaseTexture", pbr.metallic_texture.is_some().into());
            }
        }
    }
} */
