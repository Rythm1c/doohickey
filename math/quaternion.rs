use crate::math::{mat4::*, misc::*, vec3::*};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
}

pub fn quat(_x: f32, _y: f32, _z: f32, _s: f32) -> Quat {
    Quat {
        x: (_x),
        y: (_y),
        z: (_z),
        s: (_s),
    }
}

impl Quat {
    pub fn new(_x: f32, _y: f32, _z: f32, _s: f32) -> Self {
        Self {
            x: (_x),
            y: (_y),
            z: (_z),
            s: (_s),
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.s.powf(2.0)).sqrt()
    }
    pub fn unit(&self) -> Self {
        let coeff = 1.0 / self.norm();

        Self {
            x: (coeff * self.x),
            y: (coeff * self.y),
            z: (coeff * self.z),
            s: (coeff * self.s),
        }
    }
    pub fn inverse(&self) -> Self {
        Self {
            x: (-self.x),
            y: (-self.y),
            z: (-self.z),
            s: (self.s),
        }
    }

    pub fn axis(&self) -> Vec3 {
        vec3(self.x, self.y, self.z)
    }
}

use std::ops::*;
impl Sub for Quat {
    type Output = Quat;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            s: self.s - rhs.s,
        }
    }
}

impl Add for Quat {
    type Output = Quat;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            s: self.s + rhs.s,
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Quat;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            s: self.s * rhs,
        }
    }
}
impl Mul<Quat> for f32 {
    type Output = Quat;
    fn mul(self, rhs: Quat) -> Self::Output {
        Quat {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            s: self * rhs.s,
        }
    }
}
impl Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, rhs: Quat) -> Self::Output {
        Self {
            x: self.s * rhs.x + self.x * rhs.s + self.y * rhs.z - self.z * rhs.y,
            y: self.s * rhs.y + self.y * rhs.s + self.z * rhs.x - self.x * rhs.z,
            z: self.s * rhs.z + self.z * rhs.s + self.x * rhs.y - self.y * rhs.x,
            s: self.s * rhs.s - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}
/// rotate around a specified axis
/// creates a rotation matrix from a quaternion
pub fn rotate(angle: f32, axis: Vec3) -> Mat4 {
    let s = radians(angle / 2.0).sin();

    let q = Quat::new(
        axis.x * s,
        axis.y * s,
        axis.z * s,
        radians(angle / 2.0).cos(),
    );
    // first row
    let xx = 1.0 - 2.0 * (q.y.powf(2.0) + q.z.powf(2.0));
    let xy = 2.0 * (q.x * q.y - q.s * q.z);
    let xz = 2.0 * (q.x * q.z + q.s * q.y);
    // second row
    let yx = 2.0 * (q.x * q.y + q.s * q.z);
    let yy = 1.0 - 2.0 * (q.x.powf(2.0) + q.z.powf(2.0));
    let yz = 2.0 * (q.y * q.z - q.s * q.x);
    // third row
    let zx = 2.0 * (q.x * q.z - q.s * q.y);
    let zy = 2.0 * (q.y * q.z + q.s * q.x);
    let zz = 1.0 - 2.0 * (q.x.powf(2.0) + q.y.powf(2.0));

    Mat4 {
        data: [
            [xx, xy, xz, 0.0],
            [yx, yy, yz, 0.0],
            [zx, zy, zz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
