use crate::math::{mat4::*, vec3::*};
use crate::src::model::{Model, Shape};
pub struct Bone {
    parent: i32,
    pub name: String,
    pub transform: Mat4,
}
pub struct Player {
    pub id: String,
    pub model: Model,
    pub skeleton: Vec<Bone>,
}
impl Player {
    pub fn new(name: &String) -> Self {
        Self {
            skeleton: vec![],
            id: name.to_string(),
            model: Model::new(Shape::None, vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)).unwrap(),
        }
    }
}
