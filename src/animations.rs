use crate::math::{ misc::*, quaternion::*, vec3::*};

/// spin object
pub fn spin(dt: f32, angle: f32, axis: Vec3, rotation: &mut Quat) {
    let nv = axis.unit();
    rotation.x = nv.x;
    rotation.y = nv.y;
    rotation.z = nv.z;
    rotation.s += angle * dt;
}
/// rotate object around a specified center and angle per sec(velocity) along an axis
pub fn rotate_around(center: Vec3, radius: f32, angle: f32, axis: Vec3, dt: f32, pos: &mut Vec3) {
    let s = radians((angle * dt) / 2.0).sin();
    let ua = axis.unit();

    let q = Quat::new(
        ua.x * s,
        ua.y * s,
        ua.z * s,
        radians((angle * dt) / 2.0).cos(),
    );
    let unit_pos = Vec3::new(-1.0, 0.0, 0.0);
    let result = q * quat(unit_pos.x, unit_pos.y, unit_pos.z, 0.0) * q.inverse();

    *pos = result.axis() * radius + center;
}
