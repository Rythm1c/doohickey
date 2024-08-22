use crate::math::{mat4::*, vec3::*};
use crate::src::animations::*;
use crate::src::model::{Model, Shape};

pub struct Bone {
    pub name: String,
    pub parent: u8,
    pub transform: Mat4,
    pub default: Mat4,
}

pub struct Player {
    pub id: String,
    pub model: Model,
    // pub skeleton: Vec<Bone>,
    // index of current animation
    index: usize,
}

impl Player {
    pub fn new(name: &String) -> Self {
        Self {
            index: 0,
            //skeleton: Vec::new(),
            id: name.to_string(),
            model: Model::new(Shape::None, Vec3::ZERO, Vec3::ZERO).unwrap(),
        }
    }
}

/* use std::path::Path;
pub fn extract_animations(path: &Path, player: &mut Player, kind_: Animation) {
    let doc = collada::document::ColladaDocument::from_path(path).unwrap();

    let animations = collada::document::ColladaDocument::get_animations(&doc).unwrap();

    let mut skeletal_anim = SkeletalAnimation {
        bone_animations: vec![],
        kind: kind_,
    };

    for animation in &animations {
        let mut samples_: Vec<Sample> = Vec::new();
        for i in 0..animation.sample_poses.len() {
            samples_.push(Sample {
                time: animation.sample_times[i],
                transform: (Mat4 {
                    data: animation.sample_poses[i],
                }),
            });
            //println!("animation time {}", sample);
        }
        skeletal_anim.bone_animations.push(BoneAnimation {
            bone: animation.target.clone().replace("/matrix", ""),
            samples: samples_.clone(),
            index: 0,
            max_frame: samples_.len() as usize,
        });
        player.animations.push(skeletal_anim.clone());
        println!(
            "animation target {}",
            animation.target.replace("/matrix", "")
        );
    }
}
 */
