use crate::src::math::{vec2::*, vec3::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Vertex {
    pub pos: Vec3,
    pub norm: Vec3,
    pub tex: Vec2,
    pub col: Vec3,

    pub weights: [f32; 4],
    pub bone_ids: [i32; 4],
}
impl Vertex {
    pub const DEFAULT: Self = Self {
        pos: Vec3::ZERO,
        norm: Vec3::ZERO,
        tex: Vec2::ZERO,
        col: Vec3::ONE,

        weights: [0.0; 4],
        bone_ids: [-1; 4],
    };
}
