use crate::math::{quaternion::*, vec3::*};
use crate::src::transform::Transform;

// _______________________________________________________________________________________________________
// _______________________________________________________________________________________________________
// some basic animation funtions nothing fancy
// yet again lots of help from "gabor szauer - hands on c++ game animation programming packt"

/// spin object
pub fn spin(elapsed: f32, angle: f32, axis: Vec3, transform: &mut Transform) {
    transform.orientation = Quat::create(angle * elapsed, axis);
    //transform.orientation = transform.orientation.unit();
}
/// rotate object around a specified center and angle per sec(velocity) along an axis
pub fn rotate_around(
    center: Vec3,
    radius: f32,
    angle: f32,
    axis: Vec3,
    elapsed: f32,
    pos: &mut Vec3,
) {
    let q = Quat::create(angle * elapsed, axis);
    let unit_pos = Vec3::new(-1.0, 0.0, 0.0);
    let result = q * unit_pos;

    *pos = result * radius + center;
}
