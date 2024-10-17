extern crate collada;

use crate::math::mat4::Mat4;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::src::animation::clip::Clip;
use crate::src::animation::pose::Pose;
use crate::src::animation::track_transform::TransformTrack;
use crate::src::mesh::Mesh;
use crate::src::mesh::Vertex;
use crate::src::transform::Transform;

use std::path::Path;

/// loading collada files  
/// its works well enough but i wouldn't say its even good  
/// could use abit of error detection here and there plus it makes alot of assumptions  
/// the crate im using for loading collada files seems to be missing some features,  
/// but i like its simplicity and it was very easy to understand the code without any documentation  
/// i'm not very good with documentation
pub struct ColladaFile(collada::document::ColladaDocument);

impl ColladaFile {
    pub fn new(path: &Path) -> ColladaFile {
        let doc = collada::document::ColladaDocument::from_path(path).unwrap();

        ColladaFile(doc)
    }

    pub fn extract_meshes(&self) -> Vec<Mesh> {
        let mut meshes = Vec::new();

        let doc = &self.0;

        let object_set = doc.get_obj_set().unwrap();

        object_set.objects.iter().for_each(|object| {
            object.geometry.iter().for_each(|geometry| {
                geometry.mesh.iter().for_each(|primitive| {
                    meshes.push(get_polygons(primitive, object));
                });
            });
        });

        meshes
    }

    pub fn extract_rest_pose(&self) -> Pose {
        let mut pose = Pose::new();
        let skeletons = &self.0.get_skeletons().unwrap();
        //assuming theres only one skeleton
        let skeleton = &skeletons[0];
        let joints = &skeleton.joints;
        let bind_poses = &skeleton.bind_poses;

        pose.resize(skeleton.bind_poses.len());

        bind_poses.iter().enumerate().for_each(|(i, bind_pose)| {
            let mat = Mat4::from(bind_pose);
            // already in row-major
            // no need to transpose the matrix
            // crates documentation says its column-major but its not...weird
            // caused me alot of problems
            pose.joints[i] = Transform::from_mat(&mat);

            pose.parents[i] = if joints[i].is_root() {
                -1
            } else {
                joints[i].parent_index as i32
            };
        });

        pose
    }

    pub fn extract_inverse_bind_mats(&self) -> Vec<Option<Mat4>> {
        let mut inv_bind_mats = Vec::new();

        let skeletons = &self.0.get_skeletons().unwrap();
        //assuming theres only one
        let skeleton = &skeletons[0];
        let joints = &skeleton.joints;

        inv_bind_mats.resize(joints.len(), None);
        joints.iter().enumerate().for_each(|(i, joint)| {
            let inv_mat = Mat4::from(&joint.inverse_bind_pose);
            inv_bind_mats[i] = Some(inv_mat);
        });
        inv_bind_mats
    }

    pub fn extract_joint_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        let skeletons = &self.0.get_skeletons().unwrap();
        //assuming theres only one skeleton
        let skeleton = &skeletons[0];
        let joints = &skeleton.joints;

        names.resize(joints.len(), String::new());
        joints.iter().enumerate().for_each(|(i, joint)| {
            names[i] = joint.name.to_string();
        });

        names
    }

    pub fn extract_clip(&self) -> Clip {
        let mut clip = Clip::new();

        let animations = &self.0.get_animations().unwrap();

        animations.iter().for_each(|animation| {
            // target includes (name-of-joint)/(some other nonsense) hence the use of "split("/")"
            // only interested in the joints name
            let target = animation.target.split("/").next().unwrap();
            clip.tracks.push(extract_animation(
                animation,
                self.get_id(&target.to_string()).unwrap(),
            ));
        });
        clip.re_calculate_duration();

        clip
    }

    fn get_id(&self, target: &String) -> Result<u32, String> {
        let names = self.extract_joint_names();
        for (i, name) in names.iter().enumerate() {
            if target.eq(name) {
                return Ok(i as u32);
            }
        }

        return Err(String::from("couldn't find joint id in collada file!"));
    }
}

fn extract_animation(animation: &collada::Animation, id: u32) -> TransformTrack {
    let mut result = TransformTrack::new();

    result.id = id;
    result.resize(animation.sample_times.len());

    let times = &animation.sample_times;
    let poses = &animation.sample_poses;

    poses.iter().enumerate().for_each(|(i, pose)| {
        // again no need to transpose because they are row-major
        let transform = Transform::from_mat(&Mat4::from(pose));
        result.position.frames[i].m_value = transform.translation.to_array();
        result.position.frames[i].time = times[i];

        result.rotation.frames[i].m_value = transform.orientation.to_array();
        result.rotation.frames[i].time = times[i];

        result.scaling.frames[i].m_value = transform.scaling.to_array();
        result.scaling.frames[i].time = times[i];
    });

    result
}

/// reads the shapes and returns a mesh of triangles  
/// all collada files will be non indexed  
fn get_polygons(primitive: &collada::PrimitiveElement, object: &collada::Object) -> Mesh {
    let mut mesh = Mesh::default();

    match primitive {
        collada::PrimitiveElement::Polylist(polylist) => {
            polylist.shapes.iter().for_each(|shape| match shape {
                // only interested in triangles for now
                collada::Shape::Triangle(a, b, c) => {
                    // first triangle vertex
                    mesh.vertices.push(get_attributes(object, &a));
                    // second triangle vertex
                    mesh.vertices.push(get_attributes(object, &b));
                    // third triangle vertex
                    mesh.vertices.push(get_attributes(object, &c));
                }
                // not implimented yet
                collada::Shape::Line(_, _) => {}
                // not implimented yet
                collada::Shape::Point(_) => {}
            });
        }
        collada::PrimitiveElement::Triangles(_) => {}
    }

    mesh
}

/// helper to get vertex attributes
fn get_attributes(object: &collada::Object, index: &collada::VTNIndex) -> Vertex {
    let mut vertex = Vertex::DEFAULT;

    let i = index.0;
    let j = index.1.unwrap();
    let k = index.2.unwrap();

    let pos = object.vertices[i];
    vertex.pos = Vec3 {
        x: pos.x as f32,
        y: pos.y as f32,
        z: pos.z as f32,
    };

    let tex = object.tex_vertices[j];
    vertex.tex = Vec2 {
        x: tex.x as f32,
        y: tex.y as f32,
    };

    let norm = object.normals[k];
    vertex.norm = Vec3 {
        x: norm.x as f32,
        y: norm.y as f32,
        z: norm.z as f32,
    };

    let bone_ids = object.joint_weights[i].joints;
    vertex.bone_ids = bone_ids.map(|id| id as i32);

    let weights = object.joint_weights[i].weights;
    vertex.weights = weights;

    vertex
}
