use crate::math::{misc::*, quaternion::*, vec3::*};
use crate::src::object::Transform;

/// spin object
pub fn spin(dt: f32, angle: f32, axis: Vec3, transform: &mut Transform) {
    let nv = axis.unit();
    transform.axis.x = nv.x;
    transform.axis.y = nv.y;
    transform.axis.z = nv.z;
    transform.angle += angle * dt;
}
/// rotate object around a specified center and angle per sec(velocity) along an axis
pub fn rotate_around(center: Vec3, radius: f32, angle: f32, axis: Vec3, dt: f32, pos: &mut Vec3) {
    let s = radians((angle * dt) / 2.0).sin();
    let ua = axis.unit() * s;

    let q = Quat::new(ua.x, ua.y, ua.z, radians((angle * dt) / 2.0).cos());
    let unit_pos = Vec3::new(-1.0, 0.0, 0.0);
    let result = q * quat(unit_pos.x, unit_pos.y, unit_pos.z, 0.0) * q.inverse();

    *pos = result.axis() * radius + center;
}
