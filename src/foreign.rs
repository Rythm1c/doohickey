use crate::math::mat4::{inverse, Mat4};
use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;

use std::collections::HashMap;
use std::path::Path;
//...............................................................................................
//...............................................................................................
//gltf specific functions
extern crate gltf;
#[allow(dead_code)]
pub fn from_gltf(path: &Path) -> Model {
    let mut model = Model::default();

    let (document, buffers, ..) = gltf::import(path).unwrap();

    document.meshes().for_each(|mesh| {
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
                let pos = tmp_positions[i];
                let norm = tmp_normals[i];
                let mut col = Vec3::ONE;
                if i < tmp_colors.len() {
                    col = tmp_colors[i];
                }
                let mut tex = Vec2::ZERO;
                if i < tmp_colors.len() {
                    tex = tmp_tex_coords[i];
                }

                tmp_mesh.vertices.push(Vertex {
                    norm,
                    pos,
                    tex,
                    col,
                    bone_ids: [-1; 4],
                    weights: [0.0; 4],
                });
            }
            model.meshes.push(tmp_mesh.clone());
        })
    });

    model
}

//...............................................................................................
//...............................................................................................

extern crate russimp;
use russimp::scene::PostProcess;

pub fn model_from(path: &Path, color: Vec3) -> Model {
    let mut model = Model::default();

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

    let node = &scene.root.unwrap();
    let meshes = &scene.meshes;

    process_node(node, meshes, &mut model);
    model.meshes.iter_mut().for_each(|mesh| {
        mesh.vertices.iter_mut().for_each(|vert| {
            vert.col = color;
        });
    });

    model
}

fn process_node(
    node: &russimp::node::Node,
    scene_meshes: &Vec<russimp::mesh::Mesh>,
    model: &mut Model,
) {
    node.meshes.iter().for_each(|mesh| {
        let node_mesh = &scene_meshes[*mesh as usize];
        // process mesh and push to mesh stack
        let mut new_mesh = process_mesh(node_mesh);

        // push bone data to model skeleton for animations
        process_bones(
            node_mesh,
            &mut model.bone_count,
            &mut new_mesh.vertices,
            &mut model.skeleton,
        );

        model.meshes.push(new_mesh);
    });

    node.children.borrow().iter().for_each(|child| {
        process_node(child, scene_meshes, model);
    });
}
/// not getting per vertex colors for now
fn process_mesh(mesh: &russimp::mesh::Mesh) -> Mesh {
    let mut final_mesh = Mesh::default();
    // get vertices
    let mut temp_verts: Vec<Vec3> = Vec::new();
    mesh.vertices.iter().for_each(|vert| {
        let pos = vec3(vert.x, vert.y, vert.z);
        temp_verts.push(pos);
    });
    // get normals
    let mut temp_norms: Vec<Vec3> = Vec::new();
    mesh.normals.iter().for_each(|norm| {
        let normal = vec3(norm.x, norm.y, norm.z);
        temp_norms.push(normal);
    });
    // texture coordinates
    let mut temp_texs: Vec<Vec2> = Vec::new();
    mesh.texture_coords.iter().for_each(|tex_coords| {
        if let Some(texs) = tex_coords {
            texs.iter().for_each(|tex| {
                let final_tex = vec2(tex.x, tex.y);
                temp_texs.push(final_tex);
            });
        }
    });
    // assuming there is the same number of normals , texture coordinates and position vectors
    let range = 0..temp_verts.len();
    for i in range {
        let mut vertex = Vertex::DEFAULT;
        vertex.pos = temp_verts[i];
        vertex.norm = temp_norms[i];

        if i < temp_texs.len() {
            vertex.tex = temp_texs[i];
        }

        final_mesh.vertices.push(vertex);
    }

    mesh.faces.iter().for_each(|face| {
        face.0.iter().for_each(|index| {
            final_mesh.indices.push(*index);
        })
    });

    final_mesh
}
fn process_bones(
    node_mesh: &russimp::mesh::Mesh,
    counter: &mut i32,
    vertices: &mut Vec<Vertex>,
    skeleton: &mut HashMap<String, BoneInfo>,
) {
    node_mesh.bones.iter().for_each(|bone| {
        let bone_id;
        if !skeleton.contains_key(&bone.name) {
            // and also add the bone itself to the skeleton stack
            let bone_info = BoneInfo {
                id: *counter,
                offset: get_mat(&bone.offset_matrix),
            };

            skeleton.insert(bone.name.to_string(), bone_info);

            bone_id = *counter;
        } else {
            bone_id = skeleton.get(&bone.name).unwrap().id;
        }
        assert!(bone_id != -1);

        bone.weights.iter().for_each(|weight| {
            let vert_id = weight.vertex_id as usize;
            // set each vertex's bone id and weight
            // 4 because thats the max bone influence

            for i in 0..4 {
                if vertices[vert_id].bone_ids[i] < 0 {
                    vertices[vert_id].bone_ids[i] = bone_id;
                    vertices[vert_id].weights[i] = weight.weight;
                }
            }
        });

        // next bone id
        *counter += 1;
    });
}

/// convert russimp 4x4 matrix to custom matrix
pub fn get_mat(mat: &russimp::Matrix4x4) -> Mat4 {
    Mat4 {
        data: [
            [mat.a1, mat.a2, mat.a3, mat.a4],
            [mat.b1, mat.b2, mat.b3, mat.b4],
            [mat.c1, mat.c2, mat.c3, mat.c4],
            [mat.d1, mat.d2, mat.d3, mat.d4],
        ],
    }
}
