use crate::math::mat4::Mat4;
use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;

use std::path::Path;

extern crate gltf;
#[allow(dead_code)]
pub fn from_gltf(path: &str, model: &mut Model) {
    let (document, buffers, ..) = gltf::import(path).unwrap();

    for mesh in document.meshes() {
        //prepare for next batch of data
        let mut tmp_mesh = Mesh::default();

        let primitives = mesh.primitives();
        primitives.for_each(|primitive| {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            //temporary array to hold position data
            let mut tmp_positions: Vec<Vec3> = vec![];
            // extract positions
            if let Some(positions) = reader.read_positions() {
                for pos in positions {
                    tmp_positions.push(Vec3 {
                        x: pos[0],
                        y: pos[1],
                        z: pos[2],
                    });
                }
            };
            //temporary storage for normals
            let mut tmp_normals: Vec<Vec3> = vec![];
            //extract normals
            if let Some(normals) = reader.read_normals() {
                for norm in normals {
                    tmp_normals.push(Vec3 {
                        x: norm[0],
                        y: norm[1],
                        z: norm[2],
                    });
                }
            }
            //temporary storage for colors
            let mut tmp_colors: Vec<Vec3> = vec![];
            //extract normals
            if let Some(gltf::mesh::util::ReadColors::RgbF32(gltf::accessor::Iter::Standard(itr))) =
                reader.read_colors(0)
            {
                for color in itr {
                    tmp_colors.push(Vec3 {
                        x: color[0],
                        y: color[1],
                        z: color[2],
                    });
                }
            }
            //temporary storage for texure coordinates
            let mut tmp_tex_coords: Vec<Vec2> = vec![];
            //extract
            if let Some(gltf::mesh::util::ReadTexCoords::F32(gltf::accessor::Iter::Standard(itr))) =
                reader.read_tex_coords(0)
            {
                for texcoord in itr {
                    tmp_tex_coords.push(Vec2 {
                        x: texcoord[0],
                        y: texcoord[1],
                    });
                }
            }

            //extract
            if let Some(gltf::mesh::util::ReadIndices::U32(gltf::accessor::Iter::Standard(itr))) =
                reader.read_indices()
            {
                for index in itr {
                    tmp_mesh.indices.push(index);
                }
            }

            for i in 0..tmp_positions.len() {
                tmp_mesh.vertices.push(Vertex {
                    norm: tmp_normals[i],
                    pos: tmp_positions[i],
                    tex: tmp_tex_coords[i],
                    col: tmp_colors[i],
                    bone_ids: [-1; 4],
                    weights: [0.0; 4],
                });
            }
            model.meshes.push(tmp_mesh.clone());
        })
    }
}

extern crate collada;
pub fn from_collada(path: &Path) -> Model {
    let mut final_model = Model::default();

    let doc = collada::document::ColladaDocument::from_path(path).unwrap();
    // no material funtionality yet
    let object_set = &doc.get_obj_set().unwrap();
    let objects = &object_set.objects;

    objects.iter().for_each(|object| {
        object.geometry.iter().for_each(|geometry| {
            geometry.mesh.iter().for_each(|primitive| {
                final_model.meshes.push(get_polygons(primitive, object));
            });
        });
    });

    final_model.skeleton = extract_skeleton(&doc);

    final_model
}
/// helper to get mesh for collada file
fn get_polygons(primitive: &collada::PrimitiveElement, object: &collada::Object) -> Mesh {
    let mut mesh = Mesh::default();

    match primitive {
        collada::PrimitiveElement::Polylist(polylist) => {
            polylist.shapes.iter().for_each(|shape| match shape {
                // three points of one triangle
                collada::Shape::Triangle(a, b, c) => {
                    // first point
                    mesh.vertices.push(get_attributes(object, &a));
                    // second point
                    mesh.vertices.push(get_attributes(object, &b));
                    // third point
                    mesh.vertices.push(get_attributes(object, &c));
                }
                _ => {}
            });
        }
        _ => {}
    }

    mesh
}
// helper to get skeleton from file
// assumes theres only one skeleton
fn extract_skeleton(doc: &collada::document::ColladaDocument) -> Vec<BoneInfo> {
    let mut final_skeleton: Vec<BoneInfo> = Vec::new();

    let doc_skeletons = doc.get_skeletons().unwrap();
    // assuming theres only one skeleton
    doc_skeletons[0].joints.iter().for_each(|bone| {
        let name = bone.name.to_string();
        let parent = bone.parent_index as usize;
        let bind_pose = Mat4::from(&bone.inverse_bind_pose);

        final_skeleton.push(BoneInfo {
            name,
            parent,
            bind_pose,
        });
    });

    final_skeleton
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
    vertex.bone_ids = [
        bone_ids[0] as i32,
        bone_ids[1] as i32,
        bone_ids[2] as i32,
        bone_ids[3] as i32,
    ];

    let weights = object.joint_weights[i].weights;
    vertex.weights = weights;

    vertex
}
