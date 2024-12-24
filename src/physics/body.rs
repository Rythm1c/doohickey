use crate::src::math::vec3::*;

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct Sphere {
    pub radius: f32,
    pub positon: Vec3,
}

pub enum BoundingVolume {
    Sphere,
    AABB,
    OBB,
    Capsule,
}
