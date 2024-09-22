// tried using the russimp crate but it had alot of issues so had to switch to parsers for specific file formats to 
// make sure they were loaded correctly and changed the whole animation implimentation which took several life times
// this code only worked on a few very specific gltf files and never animated any fbx files properly, always ended up deformed
// also russimp could not parse collada(dae) files for some reason hmmmm.....
// maybe it was me i dont know. never again 

/*extern crate russimp;
use russimp::animation::NodeAnim;
use russimp::scene::PostProcess;

use std::collections::HashMap;
use std::path::Path;

use crate::math::{mat4::*, quaternion::*, vec3::*};

use crate::src::model::{BoneInfo, Model};
use crate::src::transform::Transform;

use crate::src::foreign::get_mat;
//..................................................................................................
//..................................................................................................

#[derive(Clone)]
struct KeyPosition {
    pos: Vec3,
    time_stamp: f32,
}
#[derive(Clone)]
struct KeyRotation {
    orientation: Quat,
    time_stamp: f32,
}
#[derive(Clone)]
struct KeyScale {
    scale: Vec3,
    time_stamp: f32,
}
#[derive(Clone)]
struct Bone {
    positions: Vec<KeyPosition>,
    rotations: Vec<KeyRotation>,
    scalings: Vec<KeyScale>,

    local_transform: Mat4,
    name: String,
}
impl Bone {
    fn new(name: &String, channel: &NodeAnim) -> Self {
        // get key positions
        let mut positions: Vec<KeyPosition> = Vec::new();
        channel.position_keys.iter().for_each(|position_key| {
            let pos = vec3(
                position_key.value.x,
                position_key.value.y,
                position_key.value.z,
            );
            let time_stamp = position_key.time as f32;

            positions.push(KeyPosition { pos, time_stamp })
        });

        // get key rotations
        let mut rotations: Vec<KeyRotation> = Vec::new();
        channel.rotation_keys.iter().for_each(|rotation_key| {
            let orientation = quat(
                rotation_key.value.x,
                rotation_key.value.y,
                rotation_key.value.z,
                rotation_key.value.w,
            );
            let time_stamp = rotation_key.time as f32;

            rotations.push(KeyRotation {
                orientation,
                time_stamp,
            })
        });

        // get key scalings
        let mut scalings: Vec<KeyScale> = Vec::new();
        channel.scaling_keys.iter().for_each(|scaling_key| {
            let scale = vec3(
                scaling_key.value.x,
                scaling_key.value.y,
                scaling_key.value.z,
            );
            let time_stamp = scaling_key.time as f32;
            //println!("time {}", time_stamp);
            scalings.push(KeyScale { scale, time_stamp })
        });

        Self {
            positions,
            rotations,
            scalings,
            local_transform: Mat4::IDENTITY,
            name: name.to_string(),
        }
    }
    fn update(&mut self, current_time: f32) {
        let translation = self.interpolate_pos(current_time);
        let rotation = self.interpolate_rotation(current_time);
        let scaling = self.interpolate_scale(current_time);

        self.local_transform = translation * rotation * scaling;
    }

    fn interpolate_pos(&self, current_time: f32) -> Mat4 {
        if 1 == self.positions.len() {
            return translate(&self.positions[0].pos);
        }

        let p0 = self.get_pos_index(current_time);
        let p1 = p0 + 1;
        let scale_factor = get_scale_factor(
            self.positions[p0].time_stamp,
            self.positions[p1].time_stamp,
            current_time,
        );
        let final_pos = mix(self.positions[p0].pos, self.positions[p1].pos, scale_factor);

        translate(&final_pos)
    }

    fn interpolate_scale(&self, current_time: f32) -> Mat4 {
        if 1 == self.scalings.len() {
            return scale(&self.scalings[0].scale);
        }

        let s0 = self.get_scaling_index(current_time);
        let s1 = s0 + 1;
        let scale_factor = get_scale_factor(
            self.scalings[s0].time_stamp,
            self.scalings[s1].time_stamp,
            current_time,
        );
        let finale_scale = mix(
            self.scalings[s0].scale,
            self.scalings[s1].scale,
            scale_factor,
        );

        scale(&finale_scale)
    }

    fn interpolate_rotation(&self, current_time: f32) -> Mat4 {
        if 1 == self.rotations.len() {
            return self.rotations[0].orientation.to_mat();
        }

        let r0 = self.get_rotation_index(current_time);
        let r1 = r0 + 1;
        let scale_factor = get_scale_factor(
            self.rotations[r0].time_stamp,
            self.rotations[r1].time_stamp,
            current_time,
        );
        let final_quat = self.rotations[r0]
            .orientation
            .nlerp(self.rotations[r1].orientation, scale_factor);

        final_quat.to_mat()
    }

    fn get_pos_index(&self, current_time: f32) -> usize {
        for index in 1..(self.positions.len() - 1) {
            if current_time < self.positions[index + 1].time_stamp {
                return index;
            }
        }
        assert!(false);
        0
    }
    fn get_rotation_index(&self, current_time: f32) -> usize {
        for index in 1..(self.rotations.len() - 1) {
            if current_time < self.rotations[index + 1].time_stamp {
                return index;
            }
        }
        assert!(false);
        0
    }
    fn get_scaling_index(&self, current_time: f32) -> usize {
        for index in 1..(self.scalings.len() - 1) {
            if current_time < self.scalings[index + 1].time_stamp {
                return index;
            }
        }
        assert!(false);
        0
    }
}
fn get_scale_factor(last_time: f32, next_time: f32, current_time: f32) -> f32 {
    let midway_len = current_time - last_time;
    let frame_diff = next_time - last_time;

    midway_len / frame_diff
}
fn mix(a: Vec3, b: Vec3, c: f32) -> Vec3 {
    c * b + (1.0 - c) * a
}
#[derive(Clone)]
struct NodeData {
    transform: Mat4,
    name: String,
    children: Vec<NodeData>,
}
#[derive(Clone)]
pub struct Animation {
    duration: f32,
    ticks_per_sec: f32,
    bones: Vec<Bone>,
    root_node: NodeData,
    bone_infos: HashMap<String, BoneInfo>,
}

impl Animation {
    pub fn new(path: &Path, model: &mut Model) -> Self {
        let file_path = path.to_str().unwrap();
        let scene = russimp::scene::Scene::from_file(
            file_path,
            vec![
                PostProcess::Triangulate,
                PostProcess::GenerateSmoothNormals,
                PostProcess::FlipUVs,
                PostProcess::FlipWindingOrder,
                PostProcess::JoinIdenticalVertices,
                PostProcess::OptimizeGraph,
            ],
        )
        .unwrap();

        let mut bones: Vec<Bone> = Vec::new();
        let mut root_node = NodeData {
            transform: Mat4::IDENTITY,
            name: String::new(),
            children: Vec::new(),
        };

        let animation = &scene.animations[0];
        let duration = animation.duration as f32;
        let ticks_per_sec = animation.ticks_per_second as f32;
        Self::read_heirarchy(&mut root_node, &scene.root.unwrap());
        Self::get_missing_bones(animation, &mut bones, model);
        let bone_infos = model.skeleton.clone();

        Self {
            duration,
            ticks_per_sec,
            bones,
            root_node,
            bone_infos,
        }
    }

    fn read_heirarchy(dest: &mut NodeData, src: &russimp::node::Node) {
        dest.name = src.name.to_string();
        dest.transform = get_mat(&src.transformation);

        src.children.borrow().iter().for_each(|child| {
            let mut data = NodeData {
                transform: Mat4::IDENTITY,
                name: String::new(),
                children: Vec::new(),
            };

            Self::read_heirarchy(&mut data, child);

            dest.children.push(data);
        });
    }

    fn get_missing_bones(
        animation: &russimp::animation::Animation,
        bones: &mut Vec<Bone>,
        model: &mut Model,
    ) {
        let bone_infos = &mut model.skeleton;

        animation.channels.iter().for_each(|channel| {
            let name = &channel.name;

            if !bone_infos.contains_key(name) {
                bone_infos.insert(
                    name.to_string(),
                    BoneInfo {
                        id: model.bone_count,
                        offset: Mat4::IDENTITY,
                    },
                );
                model.bone_count += 1;
            }

            bones.push(Bone::new(&name.to_string(), channel));
        });
    }
}

pub struct Animator {
    current_time: f32,
    current_animation: Animation,
    pub final_mats: Vec<Mat4>,
}
impl Animator {
    pub fn new(animation: &Animation) -> Self {
        let current_animation = animation.clone();
        let mut final_mats: Vec<Mat4> = Vec::with_capacity(100);

        for _ in 0..200 {
            final_mats.push(Mat4::IDENTITY);
        }
        Self {
            current_time: 0.0,

            current_animation,
            final_mats,
        }
    }
    pub fn update_animation(&mut self, delta: f32) {
        self.current_time += self.current_animation.ticks_per_sec * delta;
        self.current_time %= self.current_animation.duration;
        let root_node = self.current_animation.root_node.clone();
        self.calculate_transforms(&root_node, &Mat4::IDENTITY);
    }
    pub fn play_animation(&mut self, animation: &Animation) {
        self.current_time = 0.0;
        self.current_animation = animation.clone();
    }
    fn calculate_transforms(&mut self, node: &NodeData, parent_mat: &Mat4) {
        let name = &node.name;
        let mut node_transform = node.transform;

        self.current_animation.bones.iter_mut().for_each(|bone| {
            if bone.name.contains(name) {
                bone.update(self.current_time);
                //let new_mat;
                node_transform = bone.local_transform;
            };
        });

        let global_transform = *parent_mat * node_transform;

        if self.current_animation.bone_infos.contains_key(name) {
            let bone_info = self.current_animation.bone_infos.get(name).unwrap();
            let index = bone_info.id;
            let offset = bone_info.offset;
            self.final_mats[index as usize] = global_transform * offset;
        };

        node.children.iter().for_each(|child| {
            self.calculate_transforms(child, &global_transform);
        });
    }
}

//..................................................................................................
//..................................................................................................
//some basic animation funtions nothing fancy

/// spin object
pub fn spin(elapsed: f32, angle: f32, axis: Vec3, transform: &mut Transform) {
    transform.orientation = Quat::create(angle * elapsed, axis);
}
/// rotate object around a specified center and angle per sec(velocity) along an axis
pub fn rotate_around(
    center: Vec3,
    radius: f32,
    angle: f32,
    axis: Vec3,
    elapsed: f32,
    pos: &mut Vec3,
) {
    let q = Quat::create(angle * elapsed, axis);
    let unit_pos = Vec3::new(-1.0, 0.0, 0.0);
    let result = q * quat(unit_pos.x, unit_pos.y, unit_pos.z, 0.0) * q.inverse();

    *pos = result.axis() * radius + center;
}
*/
