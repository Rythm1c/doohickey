use crate::src::animation::pose::Pose;
use crate::src::math::mat4::{transpose, Mat4};
use crate::src::math::transform::Transform;
use crate::src::math::{quaternion::*, vec2::*, vec3::*};
use crate::src::renderer::mesh::*;

use crate::src::animation::clip::Clip;
use crate::src::animation::curves::Interpolation;
use crate::src::animation::frame::{QuaternionFrame, VectorFrame};
use crate::src::animation::track_transform::TransformTrack;
//use crate::src::texture::Texture;
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

        GltfFile(document, buffers, images)
    }

    pub fn extract_meshes(&self, meshes: &mut Vec<Mesh>) {
        //let mut meshes = Vec::new();

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

                let pbr_info = &primitive.material().pbr_metallic_roughness();
                let color = pbr_info.base_color_factor();

                println!("base color {:?}", color);

                match pbr_info.base_color_texture() {
                    Some(texture) => match texture.texture().source().source() {
                        gltf::image::Source::Uri { uri, .. } => {
                            println!("texture source {}", uri);
                        }

                        _ => {
                            println!("texture source not surported!")
                        }
                    },
                    None => {
                        println!("primitive contains no texture!")
                    }
                }

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

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

                // alittle cheating
                // have to come up with a better way of handling materials
                // might impliment a pbr system
                for vert in &mut mesh.vertices {
                    vert.col = vec3(color[0], color[1], color[2])
                }

                //extract indices
                if let Some(indices) = reader.read_indices() {
                    mesh.indices = indices.into_u32().collect();
                }

                meshes.push(mesh);
            });
        });
    }

    pub fn extract_textures(&self) {
        let document = &self.0;

        document
            .textures()
            .for_each(|texture| match texture.source().source() {
                gltf::image::Source::Uri { uri, .. } => {
                    println!("{}", uri);
                }
                _ => {}
            });
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
    ///
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

    fn extract_animation(
        &self,
        channel: &gltf::animation::Channel,
        track_transform: &mut TransformTrack,
    ) {
        let buffers = &self.1;

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
    }

    pub fn extract_animations(&self) -> Vec<Clip> {
        let document = &self.0;

        let mut clips = Vec::new();
        document.animations().for_each(|animation| {
            let mut clip = Clip::new();
            clip.name = animation.name().unwrap().to_string();

            animation.channels().for_each(|channel| {
                //check if track exists

                // bool variable to escape the horrors of the borrow checker
                let mut exists = false;
                let mut track_index = 0;

                for (i, track) in clip.tracks.iter().enumerate() {
                    if (track.id as usize) == channel.target().node().index() {
                        exists = true;

                        track_index = i;

                        break;
                    }
                }

                if exists {
                    //if it does then modify the existing one
                    self.extract_animation(&channel, &mut clip.tracks[track_index]);
                } else {
                    //if it doesn't create a new track
                    let sampler = &channel.sampler();

                    let mut interpolation = Interpolation::Constant;
                    if sampler.interpolation() == gltf::animation::Interpolation::Linear {
                        interpolation = Interpolation::Linear;
                    } else if sampler.interpolation() == gltf::animation::Interpolation::CubicSpline
                    {
                        interpolation = Interpolation::Cubic;
                    }

                    let mut new_track: TransformTrack = TransformTrack::new();

                    new_track.id = channel.target().node().index() as u32;

                    new_track.position.interpolation = interpolation;
                    new_track.rotation.interpolation = interpolation;
                    new_track.scaling.interpolation = interpolation;

                    self.extract_animation(&channel, &mut new_track);
                    clip.tracks.push(new_track);
                }
            });

            clip.re_calculate_duration();

            clips.push(clip);
        });

        clips
    }
}
