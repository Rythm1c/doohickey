use crate::math::{mat4::*, quaternion::*, vec3::*};

#[derive(Clone)]
pub struct Transform {
    pub pos: Vec3,

    pub size: Vec3,
    //for rotations
    pub orientation: Quat,
}

impl Transform {
    pub const DEFAULT: Self = Self {
        pos: Vec3::ZERO,
        orientation: Quat::ZERO,

        size: Vec3::ONE,
    };
    pub fn new(size: Vec3, pos: Vec3) -> Self {
        Self {
            pos,
            size,
            orientation: Quat::ZERO,
        }
    }
    pub fn get(&mut self) -> Mat4 {
        let translation = translate(&self.pos);
        let rotation = self.orientation.to_mat();
        let resize = scale(&self.size);

        translation * rotation * resize
    }

    pub fn lerp(&self, other: &Self, factor: f32) -> Transform {
        Self {
            pos: mix(self.pos, other.pos, factor),
            size: mix(self.size, other.size, factor),
            orientation: self.orientation.nlerp(other.orientation, factor),
        }
    }

    pub fn from_mat(mat: &Mat4) -> Self {
        let mut transform = Self::DEFAULT;

        let translation = Vec3 {
            x: mat.data[0][3],
            y: mat.data[1][3],
            z: mat.data[2][3],
        };
        
        let orientation = mat.to_quat();

        transform.pos = translation;
        transform.orientation = orientation;

        transform
    }
}
fn mix(a: Vec3, b: Vec3, c: f32) -> Vec3 {
    a * (1.0 - c) + c * b
}
