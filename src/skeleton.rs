use crate::src::animation::pose::Pose;

use crate::math::mat4::Mat4;

#[derive(Clone)]
pub struct Skeleton {
    pub rest_pose: Pose,
    pub inverse_bind_pose: Vec<Mat4>,
    pub joint_names: Vec<String>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            rest_pose: Pose::new(),
            inverse_bind_pose: Vec::new(),
            joint_names: Vec::new(),
        }
    }
}
