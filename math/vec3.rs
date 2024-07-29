use crate::math::misc::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(_x: f32, _y: f32, _z: f32) -> Self {
        Self {
            x: _x,
            y: _y,
            z: _z,
        }
    }
    // vector length
    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    // get the vector normalized
    pub fn unit(&self) -> Self {
        let inverse_legth = 1.0 / self.len();
        Self {
            x: self.x * inverse_legth,
            y: self.y * inverse_legth,
            z: self.z * inverse_legth,
        }
    }
}

pub fn vec3(_x: f32, _y: f32, _z: f32) -> Vec3 {
    Vec3::new(_x, _y, _z)
}
// dot product with another vector
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}
// reflect a vector around a normal
pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    *incident - 2.0 * dot(incident, normal) * *normal
}
//get the cross product between two vectors
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn clamp_vec3(v: &Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
    Vec3::new(
        clamp(v.x, min.x, max.x),
        clamp(v.y, min.y, max.y),
        clamp(v.z, min.z, max.z),
    )
}
use std::ops::*;
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
