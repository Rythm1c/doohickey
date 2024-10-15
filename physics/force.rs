use crate::math::vec3::*;

/// very simple gravity function for applying a gravitational force/pull  
/// not much going on
pub fn gravity(v: &mut Vec3) {
    let acc = vec3(0.0, -2e-2, 0.0);
    *v = *v + acc;
}
