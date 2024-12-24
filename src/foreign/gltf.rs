use crate::src::animation::skeleton::Skeleton;
use crate::src::math::mat4::{transpose, Mat4};
use crate::src::math::transform::Transform;
use crate::src::math::{quaternion::*, vec2::*, vec3::*};

use crate::src::renderer::ebo::Ebo;
use crate::src::renderer::mesh::Mesh;
use crate::src::renderer::model::Model;
use crate::src::renderer::texture::Texture;
use crate::src::renderer::vertex::Vertex;

use crate::src::animation::clip::Clip;
use crate::src::animation::curves::Interpolation;
use crate::src::animation::frame::{QuaternionFrame, VectorFrame};
use crate::src::animation::pose::Pose;
use crate::src::animation::track_transform::TransformTrack;

use std::fs;
use std::path::Path;
//_______________________________________________________________________________________________
//_______________________________________________________________________________________________
// gltf loader definations
// not perfect but works well enough for most files
// still a work in progress
extern crate gltf;

pub struct Gltf {
    parent_folder: String,
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
}

impl Gltf {
    pub fn new(parent: &Path) -> Gltf {
        let paths = fs::read_dir(parent).unwrap();

        let mut gltf_file = String::new();
        for entry in paths {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension.eq("gltf") || extension.eq("glb") {
                    gltf_file = String::from(path.to_str().unwrap());
                }
            }
        }

        let (document, buffers, _) = gltf::import(gltf_file).unwrap();

        let parent_folder = String::from(parent.to_str().unwrap());
        Gltf {
            parent_folder,
            document,
            buffers,
        }
    }

    pub fn populate_model(&self, model: &mut Model) {
        self.extract_meshes_and_textures(&mut model.meshes, &mut model.textures);
        self.extract_skeleton(&mut model.skeleton);
        self.extract_animations(&mut model.animations);
    }

    //parent_folder for extracting any textures found
    pub fn extract_meshes_and_textures(&self, meshes: &mut Vec<Mesh>, textures: &mut Vec<Texture>) {
        let mut skins = Vec::new();
        skins.resize(self.document.skins().count(), Vec::new());

        self.document.skins().for_each(|skin| {
            let mut joints = Vec::new();
            skin.joints().for_each(|joint| {
                joints.push(joint.index() as i32);
            });
            skins[0] = joints;
        });

        let mut texture_ids = Vec::new();

        self.document.meshes().for_each(|mesh| {
            let primitives = mesh.primitives();
            //assuming it only contains one skin
            let ids = &skins[0];

            primitives.for_each(|primitive| {
                //prepare for next batch of data
                let mut mesh = Mesh::default();

                let pbr_info = &primitive.material().pbr_metallic_roughness();
                let color = pbr_info.base_color_factor();
                // uhhhh...
                // good enough i guess
                // should probably change my code architecture to load textures better
                if let Some(texture) = pbr_info.base_color_texture() {
                    match texture.texture().source().source() {
                        gltf::image::Source::Uri { uri, .. } => {
                            if texture_ids.contains(&texture.texture().index()) {
                                let index = texture_ids
                                    .iter()
                                    .position(|&v| v == texture.texture().index())
                                    .unwrap();

                                mesh.texture = Some(textures[index].clone());
                            } else {
                                let mut tex = Texture::new();
                                let parent_folder = Path::new(&self.parent_folder[..]);

                                tex.from(parent_folder.join(uri).as_path());

                                mesh.texture = Some(tex.clone());

                                textures.push(tex);
                                texture_ids.push(texture.texture().index());
                            }
                        }

                        _ => {
                            println!("texture source not surported!")
                        }
                    }
                }

                let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));

                // extract positions
                if let Some(positions) = reader.read_positions() {
                    positions.for_each(|pos| {
                        mesh.vao.vertices.push(Vertex {
                            pos: Vec3::from(&pos),
                            ..Vertex::DEFAULT
                        });
                    });
                };

                //extract normals
                if let Some(normals) = reader.read_normals() {
                    normals.enumerate().for_each(|(i, norm)| {
                        mesh.vao.vertices[i].norm = Vec3::from(&norm);
                    });
                }

                //extract colors
                if let Some(colors) = reader.read_colors(0) {
                    colors.into_rgb_f32().enumerate().for_each(|(i, color)| {
                        mesh.vao.vertices[i].col = Vec3::from(&color);
                    });
                }
                //extract texture coordinates
                if let Some(texels) = reader.read_tex_coords(0) {
                    texels.into_f32().enumerate().for_each(|(i, texel)| {
                        mesh.vao.vertices[i].tex = Vec2::from(&texel);
                    });
                }

                //extract weights
                if let Some(weights) = reader.read_weights(0) {
                    weights.into_f32().enumerate().for_each(|(i, weight)| {
                        mesh.vao.vertices[i].weights = weight;
                    });
                }

                //extract bone ids
                if let Some(boneids) = reader.read_joints(0) {
                    boneids.into_u16().enumerate().for_each(|(i, batch)| {
                        if ids.len() > 0 {
                            mesh.vao.vertices[i].bone_ids = [
                                ids[batch[0] as usize],
                                ids[batch[1] as usize],
                                ids[batch[2] as usize],
                                ids[batch[3] as usize],
                            ];
                        } else {
                            mesh.vao.vertices[i].bone_ids = batch.map(|id| id as i32);
                        }
                    });
                }

                // alittle cheating
                // have to come up with a better way of handling materials
                // might impliment a pbr system
                for vert in &mut mesh.vao.vertices {
                    vert.col = vec3(color[0], color[1], color[2])
                }

                //extract indices
                if let Some(indices) = reader.read_indices() {
                    mesh.ebo = Some(Ebo::new());
                    mesh.ebo.as_mut().unwrap().indices = indices.into_u32().collect();
                }

                meshes.push(mesh);
            });
        });
    }

    fn extract_skeleton(&self, skeleton: &mut Skeleton) {
        self.extract_inverse_bind_mats(&mut skeleton.inverse_bind_pose);
        self.extract_rest_pose(&mut skeleton.rest_pose);
        self.extract_joint_names(&mut skeleton.joint_names);
    }

    //_______________________________________________________________________________________________
    //_______________________________________________________________________________________________
    // pose loading function along with its helpers

    fn extract_joint_names(&self, names: &mut Vec<String>) {
        self.document.nodes().for_each(|node| {
            names.push(node.name().unwrap().to_string());
        });
    }
    //_______________________________________________________________________________________________
    //_______________________________________________________________________________________________
    /// helper for getting transform form a node
    ///
    fn get_local_transform(node: &gltf::Node) -> Transform {
        let mut result = Transform::DEFAULT;

        let transform = node.transform().decomposed();
        result.translation = Vec3::from(&transform.0);
        result.orientation = Quat::from(&transform.1);
        result.scaling = Vec3::from(&transform.2);

        result
    }

    fn extract_rest_pose(&self, pose: &mut Pose) {
        pose.resize(self.document.nodes().count());

        self.document.nodes().for_each(|node| {
            let transform = Self::get_local_transform(&node);
            pose.joints[node.index()] = transform;

            node.children().for_each(|child| {
                pose.parents[child.index()] = node.index() as i32;
            });
        });
    }

    fn extract_inverse_bind_mats(&self, inv_poses: &mut Vec<Option<Mat4>>) {
        inv_poses.resize(self.document.nodes().count(), None);

        // assumes theres only one skin
        // need to fix this
        self.document.skins().for_each(|skin| {
            let reader = skin.reader(|buffer| Some(&self.buffers[buffer.index()]));

            let mut inv_bind_mats = Vec::new();
            if let Some(inverse_bind_mats) = reader.read_inverse_bind_matrices() {
                inv_bind_mats = inverse_bind_mats.collect();
            }

            for (i, joint) in skin.joints().enumerate() {
                let inv_mat = &Mat4::from(&inv_bind_mats[i]);

                inv_poses[joint.index()] = Some(transpose(inv_mat));
            }
        });
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
        let mut key_frames_times: Vec<f32> = Vec::new();
        let reader = channel.reader(|buffer| Some(&self.buffers[buffer.index()]));

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

    fn extract_animations(&self, clips: &mut Vec<Clip>) {
        self.document.animations().for_each(|animation| {
            let mut clip = Clip::new();
            clip.name = animation.name().unwrap().to_string();

            animation.channels().for_each(|channel| {
                //check if track exists

                // bool variable to escape the horrors of the borrow checker
                let mut exists = false;

                for (i, track) in clip.tracks.iter().enumerate() {
                    if (track.id as usize) == channel.target().node().index() {
                        //if it does then modify the existing one
                        self.extract_animation(&channel, &mut clip.tracks[i]);
                        exists = true;
                        break;
                    }
                }

                if !exists {
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
    }
}
