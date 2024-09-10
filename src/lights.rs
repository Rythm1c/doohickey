extern crate gl;
use crate::math::{mat4::*, vec3::*};
use crate::src::shadows;

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
