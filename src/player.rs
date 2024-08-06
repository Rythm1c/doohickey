use crate::math::{mat4::*, vec3::*};
use crate::src::animations::{BoneAnimations, Sample};
use crate::src::model::{Model, Shape};

pub struct Bone {
    pub parent: u8,
    pub name: String,
    pub transform: Mat4,
}
pub enum Animation {
    Static,  // no animation
    Default, // idle
    Running,
}
pub struct Player {
    pub id: String,
    pub model: Model,
    pub skeleton: Vec<Bone>,
    pub animations: Vec<BoneAnimations>,
    pub current_animation: Animation,
}

impl Player {
    pub fn new(name: &String) -> Self {
        Self {
            current_animation: Animation::Static,
            skeleton: vec![],
            id: name.to_string(),
            animations: vec![],
            model: Model::new(Shape::None, vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)).unwrap(),
        }
    }
}
use std::path::Path;
pub fn extract_animations(path: &Path, player: &mut Player) {
    let doc = collada::document::ColladaDocument::from_path(path).unwrap();

    let animations = collada::document::ColladaDocument::get_animations(&doc).unwrap();
    for animation in &animations {
        let mut samples_: Vec<Sample> = Vec::new();
        for i in 0..animation.sample_times.len() {
            samples_.push(Sample {
                time: animation.sample_times[i],
                transform: Mat4 {
                    data: animation.sample_poses[i],
                },
            });
            //println!("animation time {}", sample);
        }
        player.animations.push(BoneAnimations {
            bone: animation.target.clone(),
            samples: samples_,
        });
        //println!("animation target {}", animation.target);
    }
}
