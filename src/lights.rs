extern crate gl;
use crate::math::{mat4::*, vec3::*};
use crate::src::shadows;

pub struct PointLight {
    pub pos: Vec3,
    pub col: Vec3,
}

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
}
