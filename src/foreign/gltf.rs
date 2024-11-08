use crate::math::mat4::{transpose, Mat4};
use crate::math::{quaternion::*, vec2::*, vec3::*};
use crate::src::animation::pose::Pose;
use crate::src::mesh::*;
use crate::src::transform::Transform;

use crate::src::animation::clip::Clip;
use crate::src::animation::curves::Interpolation;
use crate::src::animation::frame::{QuaternionFrame, VectorFrame};
use crate::src::animation::track_transform::TransformTrack;
use crate::src::texture::Texture;
use std::path::Path;
//_______________________________________________________________________________________________
//_______________________________________________________________________________________________
// gltf loader definations
// not perfect but works well enough for most files
// still a work in progress
extern crate gltf;

/// 0: documnet, 1: buffers, 2: images
/// planning on implimenting an image loader soon an materials
#[allow(unused)]
pub struct GltfFile(
    gltf::Document,
    Vec<gltf::buffer::Data>,
    Vec<gltf::image::Data>,
);

impl GltfFile {
    pub fn new(path: &Path) -> GltfFile {
        let path = path.to_str().unwrap();
        let (document, buffers, images) = gltf::import(path).unwrap();

        println!("information about {}", path.split("/").last().unwrap());

        println!("number of meshes {}", document.meshes().count());

        let joint_count = document.nodes().count();
        if joint_count == 0 {
            println!("no joints were found ");
        } else {
            println!("number of joints {joint_count}");
        }

        let animation_count = document.animations().count();
        if animation_count == 0 {
            println!("no animations were found");
        } else {
            println!("number of animations {animation_count}");
        }

        let skin_count = document.skins().count();
        if skin_count == 0 {
            println!("no skins were found");
        } else {
            println!("number of skins {skin_count}")
        }

        let material_count = document.materials().count();
        if material_count == 0 {
            println!("no materials found")
        } else {
            println!("number of material {material_count}");
        }

        GltfFile(document, buffers, images)
    }

    pub fn extract_meshes(&self) -> Vec<Mesh> {
        let mut meshes = Vec::new();

        let document = &self.0;
        let buffers = &self.1;

        let mut skins = Vec::new();
        skins.resize(document.skins().count(), Vec::new());

        document.skins().for_each(|skin| {
            let mut joints = Vec::new();
            skin.joints().for_each(|joint| {
                joints.push(joint.index() as i32);
            });
            skins[0] = joints;
        });

        document.meshes().for_each(|mesh| {
            let primitives = mesh.primitives();
            //assuming it only contains one skin
            let ids = &skins[0];

            primitives.for_each(|primitive| {
                //prepare for next batch of data
                let mut mesh = Mesh::default();

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                //primitive.material().index();
                // extract positions
                if let Some(positions) = reader.read_positions() {
                    positions.for_each(|pos| {
                        mesh.vertices.push(Vertex {
                            pos: Vec3::from(&pos),
                            ..Vertex::DEFAULT
                        });
                    });
                };

                //extract normals
                if let Some(normals) = reader.read_normals() {
                    normals.enumerate().for_each(|(i, norm)| {
                        mesh.vertices[i].norm = Vec3::from(&norm);
                    });
                }

                //extract colors
                if let Some(colors) = reader.read_colors(0) {
                    colors.into_rgb_f32().enumerate().for_each(|(i, color)| {
                        mesh.vertices[i].col = Vec3::from(&color);
                    });
                }
                //extract texture coordinates
                if let Some(texels) = reader.read_tex_coords(0) {
                    texels.into_f32().enumerate().for_each(|(i, texel)| {
                        mesh.vertices[i].tex = Vec2::from(&texel);
                    });
                }

                //extract weights
                if let Some(weights) = reader.read_weights(0) {
                    weights.into_f32().enumerate().for_each(|(i, weight)| {
                        mesh.vertices[i].weights = weight;
                    });
                }

                //extract bone ids
                if let Some(boneids) = reader.read_joints(0) {
                    boneids.into_u16().enumerate().for_each(|(i, batch)| {
                        if ids.len() > 0 {
                            mesh.vertices[i].bone_ids = [
                                ids[batch[0] as usize],
                                ids[batch[1] as usize],
                                ids[batch[2] as usize],
                                ids[batch[3] as usize],
                            ];
                        } else {
                            mesh.vertices[i].bone_ids = batch.map(|id| id as i32);
                        }
                    });
                }

                //extract indices
                if let Some(indices) = reader.read_indices() {
                    mesh.indices = indices.into_u32().collect();
                }

                meshes.push(mesh);
            });
        });

        meshes
    }

    pub fn extract_materials(&self) {
        let document = &self.0;
        let buffers = &self.1;
        let images = &self.2;

        document.images().for_each(|image| {
            match image.source() {
                gltf::image::Source::Uri { uri, .. } => {
                    println!("image source = {uri}");
                }
                gltf::image::Source::View { view: _, .. } => {
                    println!("image stored in buffer view")
                }
            };
        });

        /*  document.materials().for_each(|material| {
            material.emissive_texture();
        }); */
    }

    //_______________________________________________________________________________________________
    //_______________________________________________________________________________________________
    // pose loading function along with its helpers

    pub fn extract_joint_names(&self) -> Vec<String> {
        let document = &self.0;

        let mut names = Vec::new();

        document.nodes().for_each(|node| {
            names.push(node.name().unwrap().to_string());
        });

        names
    }
    //_______________________________________________________________________________________________
    //_______________________________________________________________________________________________

    /// helper for getting transform form a node
    pub fn get_local_transform(node: &gltf::Node) -> Transform {
        let mut result = Transform::DEFAULT;

        let transform = node.transform().decomposed();
        result.translation = Vec3::from(&transform.0);
        result.orientation = Quat::from(&transform.1);
        result.scaling = Vec3::from(&transform.2);

        result
    }

    pub fn extract_rest_pose(&self) -> Pose {
        let document = &self.0;

        let mut pose = Pose::new();
        pose.resize(document.nodes().count());

        document.nodes().for_each(|node| {
            let transform = Self::get_local_transform(&node);
            pose.joints[node.index()] = transform;

            node.children().for_each(|child| {
                pose.parents[child.index()] = node.index() as i32;
            });
        });

        pose
    }

    pub fn extract_inverse_bind_mats(&self) -> Vec<Option<Mat4>> {
        let document = &self.0;
        let buffers = &self.1;

        let mut inv_poses: Vec<Option<Mat4>> = Vec::new();
        inv_poses.resize(document.nodes().count(), None);

        // assumes theres only one skin
        // need to fix this
        document.skins().for_each(|skin| {
            let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut inv_bind_mats = Vec::new();
            if let Some(inverse_bind_mats) = reader.read_inverse_bind_matrices() {
                inv_bind_mats = inverse_bind_mats.collect();
            }

            for (i, joint) in skin.joints().enumerate() {
                let inv_mat = &Mat4::from(&inv_bind_mats[i]);

                inv_poses[joint.index()] = Some(transpose(inv_mat));
            }
        });

        inv_poses
    }

    //_______________________________________________________________________________________________
    //_______________________________________________________________________________________________
    // loading animations data
    // its been 2 weeks now typing without seeing any pretty animation on the screen
    // this is starting to feel like a big mistake
    // skill issue or not i dont care just please fuckking WORK!
    // it finally works btw :)

    fn extract_animation(&self, channel: &gltf::animation::Channel) -> TransformTrack {
        let buffers = &self.1;
        let sampler = &channel.sampler();

        let mut interpolation = Interpolation::Constant;
        if sampler.interpolation() == gltf::animation::Interpolation::Linear {
            interpolation = Interpolation::Linear;
        } else if sampler.interpolation() == gltf::animation::Interpolation::CubicSpline {
            interpolation = Interpolation::Cubic;
        }

        let mut key_frames_times: Vec<f32> = Vec::new();
        let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));

        if let Some(inputs) = reader.read_inputs() {
            match inputs {
                gltf::accessor::Iter::Standard(times) => {
                    key_frames_times = times.collect();
                }
                gltf::accessor::Iter::Sparse(_) => {
                    println!("sparce key frames not supported");
                }
            }
        };

        let mut track_transform = TransformTrack::new();

        track_transform.id = channel.target().node().index() as u32;

        track_transform.position.interpolation = interpolation;
        track_transform.rotation.interpolation = interpolation;
        track_transform.scaling.interpolation = interpolation;

        if let Some(outputs) = reader.read_outputs() {
            match outputs {
                gltf::animation::util::ReadOutputs::Translations(translations) => {
                    track_transform
                        .position
                        .frames
                        .resize(translations.len(), VectorFrame::new());
                    translations.enumerate().for_each(|(i, translation)| {
                        track_transform.position.frames[i].m_value = translation;
                        track_transform.position.frames[i].time = key_frames_times[i];
                    });
                }
                gltf::animation::util::ReadOutputs::Rotations(rotations) => {
                    let rotations = rotations.into_f32();
                    track_transform
                        .rotation
                        .frames
                        .resize(rotations.len(), QuaternionFrame::new());
                    rotations.enumerate().for_each(|(i, rotation)| {
                        track_transform.rotation.frames[i].m_value = rotation;
                        track_transform.rotation.frames[i].time = key_frames_times[i];
                    });
                }
                gltf::animation::util::ReadOutputs::Scales(scalings) => {
                    track_transform
                        .scaling
                        .frames
                        .resize(scalings.len(), VectorFrame::ONE);
                    scalings.enumerate().for_each(|(i, scaling)| {
                        track_transform.scaling.frames[i].m_value = scaling;
                        track_transform.scaling.frames[i].time = key_frames_times[i];
                    });
                }

                gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {}
            }
        }

        track_transform
    }

    pub fn extract_animations(&self) -> Vec<Clip> {
        let document = &self.0;

        let mut clips = Vec::new();
        document.animations().for_each(|animation| {
            let mut clip = Clip::new();
            clip.name = animation.name().unwrap().to_string();
            animation.channels().for_each(|channel| {
                clip.tracks.push(self.extract_animation(&channel));
            });
            clip.re_calculate_duration();
            clips.push(clip);
        });

        clips
    }
}
