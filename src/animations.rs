use crate::math::{quaternion::*, vec3::*};

use crate::src::object::Transform;

/// spin object
pub fn spin(elapsed: f32, angle: f32, axis: Vec3, transform: &mut Transform) {
    transform.orientation = Quat::create(angle * elapsed, axis);
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
    let result = q * quat(unit_pos.x, unit_pos.y, unit_pos.z, 0.0) * q.inverse();

    *pos = result.axis() * radius + center;
}
