use crate::math::mat4::Mat4;
use crate::math::vec3::Vec3;

use crate::src::animation::clip::Clip;
use crate::src::animation::pose::Pose;
use crate::src::model::Model;
use crate::src::skeleton::Skeleton;
use crate::src::transform::Transform;

#[derive(Clone)]
pub struct Object {
    pub model: Model,
    pub transform: Transform,
    pub velocity: Vec3,
    pub animations: Vec<Clip>,
    pub skeleton: Skeleton,
    pub current_anim: usize,
    pub play_animation: bool,
    pub final_pose: Pose,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::DEFAULT,
            model: Model::default(),
            velocity: Vec3::ZERO,
            skeleton: Skeleton::new(),
            animations: Vec::new(),
            play_animation: false,
            current_anim: 0,
            final_pose: Pose::new(),
        }
    }
    pub fn change_pos(&mut self, n_pos: Vec3) -> &mut Self {
        self.transform.translation = n_pos;
        self
    }
    pub fn change_size(&mut self, n_size: Vec3) -> &mut Self {
        self.transform.scaling = n_size;
        self
    }

    pub fn update_model(&mut self, model: Model) {
        self.model = model;
    }

    pub fn update_pos_with_velocity(&mut self) {
        self.transform.translation = self.transform.translation + self.velocity;
    }

    pub fn update_animation(&mut self, time: f32) {
        if self.play_animation {
            self.final_pose = self.skeleton.rest_pose.clone();
            // extract animation for each joint(bone)
            self.animations[self.current_anim].sample(&mut self.final_pose, time);
        }
    }

    pub fn get_pose(&mut self) -> Vec<Mat4> {
        let mut final_mats = Vec::new();

        let len = self.skeleton.rest_pose.joints.len();
        //black holders for mats which arent being used
        final_mats.resize(len, Mat4::IDENTITY);

        if self.play_animation {
            // the match statements are a slight attempt at optimization, to reduce the amount of matrix multiplication going on ofcourse
            // only multiply with the joint that are actually sent to the shader
            // not sure if this is significant though
            for i in 0..len {
                if let Some(inverse_pose) = self.skeleton.inverse_bind_pose[i] {
                    // only get global transforms for joints being used
                    // to reduce iterations perfomed ofcourse
                    let world = self.final_pose.get_global_tranform(i);

                    final_mats[i] = world.to_mat() * inverse_pose;
                } else {
                    // do nothing if joint contains no inverse bind pose
                    // final_mats[i] = world.to_mat() ;
                }
            }
        } else {
            for i in 0..len {
                // only get global transforms for joints being used
                if let Some(inverse_pose) = self.skeleton.inverse_bind_pose[i] {
                    let world = self.skeleton.rest_pose.get_global_tranform(i);

                    final_mats[i] = world.to_mat() * inverse_pose;
                } else {
                    // do nothing if joint contains no inverse bind pose
                    // final_mats[i] = world.to_mat();
                }
            }
        }

        final_mats
    }
}
