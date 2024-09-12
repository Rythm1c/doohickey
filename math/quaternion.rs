use crate::math::{mat4::*, vec3::*};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
}

pub fn quat(x: f32, y: f32, z: f32, s: f32) -> Quat {
    Quat { x, y, z, s }
}

impl Quat {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        s: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32, s: f32) -> Self {
        Self { x, y, z, s }
    }
    /// halves the angle and creates a quaternion from it and the specified axis  
    /// also axis is normalized so no worries
    pub fn create(angle: f32, axis: Vec3) -> Self {
        let s = radians(angle / 2.0).sin();
        let c = radians(angle / 2.0).cos();

        let unit_axis = axis.unit();

        let x = s * unit_axis.x;
        let y = s * unit_axis.y;
        let z = s * unit_axis.z;
        let s = c;

        Self { x, y, z, s }
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

    pub fn nlerp(&self, other: Self, c: f32) -> Quat {
        (*self + (other - *self) * c).unit()
    }

    pub fn axis(&self) -> Vec3 {
        vec3(self.x, self.y, self.z)
    }

    /// rotate around a specified axis
    /// creates a rotation matrix from a quaternion
    pub fn to_mat(&self) -> Mat4 {
        // first row
        let xx = 1.0 - 2.0 * (self.y.powf(2.0) + self.z.powf(2.0));
        let xy = 2.0 * (self.x * self.y - self.s * self.z);
        let xz = 2.0 * (self.x * self.z + self.s * self.y);
        // second row
        let yx = 2.0 * (self.x * self.y + self.s * self.z);
        let yy = 1.0 - 2.0 * (self.x.powf(2.0) + self.z.powf(2.0));
        let yz = 2.0 * (self.y * self.z - self.s * self.x);
        // third row
        let zx = 2.0 * (self.x * self.z - self.s * self.y);
        let zy = 2.0 * (self.y * self.z + self.s * self.x);
        let zz = 1.0 - 2.0 * (self.x.powf(2.0) + self.y.powf(2.0));

        Mat4 {
            data: [
                [xx, xy, xz, 0.0],
                [yx, yy, yz, 0.0],
                [zx, zy, zz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

use std::ops::*;

use super::misc::radians;
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
