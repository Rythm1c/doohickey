use crate::math::vec3::Vec3;
use crate::src::model::Model;
use crate::src::transform::Transform;
//#[allow(dead_code)]

#[derive(Clone)]
pub struct Object {
    pub model: Model,
    pub transform: Transform,
    pub velocity: Vec3,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::DEFAULT,
            model: Model::default(),
            velocity: Vec3::ZERO,
        }
    }
    pub fn change_pos(&mut self, n_pos: Vec3) -> &mut Self {
        self.transform.pos = n_pos;
        self
    }
    pub fn change_size(&mut self, n_size: Vec3) -> &mut Self {
        self.transform.size = n_size;
        self
    }

    pub fn update_model(&mut self, model: Model) {
        self.model = model;
    }

    pub fn update_pos_with_velocity(&mut self) {
        self.transform.pos = self.transform.pos + self.velocity;
    }
}
