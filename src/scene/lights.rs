extern crate gl;
use crate::src::math::{mat4::*, vec3::*};
use crate::src::renderer::shaders;
use crate::src::renderer::shadows;

#[derive(Clone, Copy)]
pub struct PointLight {
    pub pos: Vec3,
    pub col: Vec3,
}
// only directional light shadow support at the moment
// might add point light shadows in the future who knows ¯\_(ツ)_/¯
pub struct DirectionalLight {
    pub dir: Vec3,
    pub color: Vec3,
    pub shadows: shadows::Shadow,
}

impl DirectionalLight {
    pub fn default() -> Self {
        Self {
            shadows: shadows::Shadow::new(800, 600),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.5, -0.5, 1.0),
        }
    }
    /// for rendering to shadow map
    pub fn get_view(&self) -> Mat4 {
        look_at(&vec3(0.0, 0.0, 0.0), &self.dir, &vec3(0.0, 1.0, 0.0))
    }
    pub fn get_projection(&self) -> Mat4 {
        orthogonal(-100.0, 100.0, 100.0, -100.0, -100.0, 100.0)
    }

    pub fn transform(&self) -> Mat4 {
        self.get_projection() * self.get_view()
    }
}

/// send point light to shaders point light array
pub fn pl_to_shader(light: PointLight, shader: &mut shaders::Program, i: usize) {
    let pos = format!("pointLights[{i}].position");
    let col = format!("pointLights[{i}].color");
    shader.update_vec3(pos.as_str(), light.pos);
    shader.update_vec3(col.as_str(), light.col);
}
