use crate::math::{misc::*, vec3::*};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

pub fn mat4(
    xx: f32,
    xy: f32,
    xz: f32,
    xw: f32,
    yx: f32,
    yy: f32,
    yz: f32,
    yw: f32,
    zx: f32,
    zy: f32,
    zz: f32,
    zw: f32,
    wx: f32,
    wy: f32,
    wz: f32,
    ww: f32,
) -> Mat4 {
    Mat4 {
        data: [
            [xx, xy, xz, xw],
            [yx, yy, yz, yw],
            [zx, zy, zz, zw],
            [wx, wy, wz, ww],
        ],
    }
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
use std::ops::*;
impl Mul<Mat4> for f32 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4 {
            data: [
                [
                    rhs.data[0][0] * self,
                    rhs.data[0][1] * self,
                    rhs.data[0][2] * self,
                    rhs.data[0][3] * self,
                ],
                [
                    rhs.data[1][0] * self,
                    rhs.data[1][1] * self,
                    rhs.data[1][2] * self,
                    rhs.data[1][3] * self,
                ],
                [
                    rhs.data[2][0] * self,
                    rhs.data[2][1] * self,
                    rhs.data[2][2] * self,
                    rhs.data[2][3] * self,
                ],
                [
                    rhs.data[3][0] * self,
                    rhs.data[3][1] * self,
                    rhs.data[3][2] * self,
                    rhs.data[3][3] * self,
                ],
            ],
        }
    }
}

/// matrix multiplication helper.
/// multiply corresponding row and column elements
fn c_r(column: usize, row: usize, m1: &Mat4, m2: &Mat4) -> f32 {
    let v1 = m1.data[column][0] * m2.data[0][row];
    let v2 = m1.data[column][1] * m2.data[1][row];
    let v3 = m1.data[column][2] * m2.data[2][row];
    let v4 = m1.data[column][3] * m2.data[3][row];

    v1 + v2 + v3 + v4
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        Self {
            data: [
                [
                    c_r(0, 0, &self, &rhs),
                    c_r(0, 1, &self, &rhs),
                    c_r(0, 2, &self, &rhs),
                    c_r(0, 3, &self, &rhs),
                ],
                [
                    c_r(1, 0, &self, &rhs),
                    c_r(1, 1, &self, &rhs),
                    c_r(1, 2, &self, &rhs),
                    c_r(1, 3, &self, &rhs),
                ],
                [
                    c_r(2, 0, &self, &rhs),
                    c_r(2, 1, &self, &rhs),
                    c_r(2, 2, &self, &rhs),
                    c_r(2, 3, &self, &rhs),
                ],
                [
                    c_r(3, 0, &self, &rhs),
                    c_r(3, 1, &self, &rhs),
                    c_r(3, 2, &self, &rhs),
                    c_r(3, 3, &self, &rhs),
                ],
            ],
        }
    }
}

pub fn translate(p: &Vec3) -> Mat4 {
    Mat4 {
        data: [
            [1.0, 0.0, 0.0, p.x],
            [0.0, 1.0, 0.0, p.y],
            [0.0, 0.0, 1.0, p.z],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
pub fn scale(s: &Vec3) -> Mat4 {
    Mat4 {
        data: [
            [s.x, 0.0, 0.0, 0.0],
            [0.0, s.y, 0.0, 0.0],
            [0.0, 0.0, s.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
//TODO: add rotation functions

/// for camera rotation
pub fn look_at(eye: &Vec3, front: &Vec3, up: &Vec3) -> Mat4 {
    // camera direction
    let cd = (*eye - *front).unit();
    // get right vector
    let cr = cross(&up, &cd).unit();
    // get up vector
    let cu = cross(&cd, &cr).unit();

    // translation vector
    let xw = -(eye.x * cr.x) - (eye.y * cr.y) - (eye.z * cr.z);
    let yw = -(eye.x * cu.x) - (eye.y * cu.y) - (eye.z * cu.z);
    let zw = -(eye.x * cd.x) - (eye.y * cd.y) - (eye.z * cd.z);

    Mat4 {
        data: [
            [cr.x, cr.y, cr.z, xw],
            [cu.x, cu.y, cu.z, yw],
            [cd.x, cd.y, cd.z, zw],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
/// l: left, r: right, n: near, f: far, t: top, b: bottom  
/// create a clipping volume from sepcified distances
pub fn frustrum(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Mat4 {
    Mat4 {
        data: [
            [(2.0 * n) / (r - l), 0.0, (r + l) / (r - l), 0.0],
            [0.0, (2.0 * n) / (t - b), (t + b) / (t - b), 0.0],
            [0.0, 0.0, -(f + n) / (f - n), (-2.0 * f * n) / (f - n)],
            [0.0, 0.0, -1.0, 0.0],
        ],
    }
}

pub fn orthogonal(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Mat4 {
    Mat4 {
        data: [
            [2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)],
            [0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)],
            [0.0, 0.0, -2.0 / (f - n), -(n + f) / (f - n)],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    let tangent = radians(fov / 2.0).tan();
    let top = near * tangent;
    let right = top * aspect_ratio;

    frustrum(-right, right, top, -top, near, far)
}
