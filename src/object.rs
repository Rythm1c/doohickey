use crate::math::{mat4::*, quaternion::*, vec3::*};
use crate::src::model::{Model, Shape};

#[derive(Clone)]
pub struct Object {
    pub model: Model,
    pub transform: Transform,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::DEFAULT,
            model: Model::DEFAULT,
        }
    }
    pub fn change_pos(&mut self, n_pos: Vec3) -> &mut Self {
        self.transform.pos = n_pos;
        self
    }
    pub fn change_shape(&mut self, n_shape: Shape) -> &mut Self {
        self.transform.shape = n_shape;
        self
    }

    pub fn update_model(&mut self, model: Model) {
        self.model = model;
    }
}

#[derive(Clone)]
pub struct Transform {
    pub pos: Vec3,
    pub velocity: Vec3,
    pub shape: Shape,
    //for rotations
    pub axis: Vec3,
    pub angle: f32,
}

impl Transform {
    const DEFAULT: Self = Self {
        pos: Vec3::ZERO,
        axis: Vec3::ZERO,
        angle: 0.0,
        velocity: Vec3::ZERO,
        shape: Shape::None,
    };
    pub fn new(shape: Shape, pos: Vec3) -> Self {
        Self {
            pos,
            shape,
            angle: 0.0,
            axis: Vec3::ZERO,
            velocity: Vec3::ZERO,
        }
    }
    pub fn get(&mut self) -> Mat4 {
        self.pos = self.pos + self.velocity;

        let translation = translate(&self.pos);

        let rotation = rotate(self.angle, self.axis);

        let size = match self.shape {
            Shape::Cube { dimensions } => scale(&dimensions),

            Shape::Sphere { radius } => scale(&vec3(radius, radius, radius)),

            Shape::None => scale(&Vec3::ZERO),
        };

        translation * rotation * size
    }
}
