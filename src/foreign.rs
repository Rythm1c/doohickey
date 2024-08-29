use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;
use std::path::Path;

extern crate gltf;
#[allow(dead_code)]
pub fn from_gltf(path: &str, model: &mut Model) {
    let (document, buffers, ..) = gltf::import(path).unwrap();

    for mesh in document.meshes() {
        //prepare for next batch of data
        let mut tmp_mesh = Mesh::DEFAULT;

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
                    })
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
                    })
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
                })
            }
            model.meshes.push(tmp_mesh.clone());
        })
    }
}

extern crate russimp;
use russimp::scene::PostProcess;

pub fn from_extern_src(path: &Path) -> Model {
    let mut model = Model::DEFAULT;

    // lets go
    let scene = russimp::scene::Scene::from_file(
        path.to_str().unwrap(),
        vec![PostProcess::Triangulate, PostProcess::FlipUVs],
    )
    .unwrap();

    let node = scene.root.clone().unwrap();

    println!("num of meshes {}", node.meshes.len());
    for mesh in node.meshes.clone() {
        let node_mesh = &scene.meshes[mesh as usize];

        model.meshes.push(process_mesh(node_mesh));
    }

    for child in node.children.borrow().iter() {
        for mesh in child.meshes.clone().into_iter() {
            let node_mesh = &scene.meshes[mesh as usize];

            model.meshes.push(process_mesh(node_mesh));
        }
    }

    model
}

fn process_mesh(mesh: &russimp::mesh::Mesh) -> Mesh {
    let mut node_mesh = Mesh::DEFAULT;

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    for vert in &mesh.vertices {
        vertices.push([vert.x, vert.y, vert.z]);
    }

    let mut normals: Vec<[f32; 3]> = Vec::new();
    for norm in &mesh.normals {
        normals.push([norm.x, norm.y, norm.z]);
    }
    let mut total_texcoords = 0;
    let mut tex_coords: Vec<[f32; 2]> = Vec::new();
    for tex in &mesh.texture_coords {
        match tex {
            Some(tex_coord) => {
                for uv in tex_coord {
                    tex_coords.push([uv.x, uv.y]);
                    total_texcoords += 1;
                }
            }
            None => println!("missing tex coords"),
        }
    }

    let mut total_colors = 0;
    let mut tmp_colors: Vec<[f32; 3]> = Vec::new();
    for colors in &mesh.colors {
        match colors {
            Some(color) => {
                for col in color {
                    tmp_colors.push([col.r, col.g, col.b]);
                    total_colors += 1;
                }
            }
            None => println!("missing colors"),
        }
    }

    let range = 0..mesh.vertices.len();
    for i in range {
        let mut vertex = Vertex::DEFAULT;
        vertex.pos = vec3(vertices[i][0], vertices[i][1], vertices[i][2]);
        vertex.norm = vec3(normals[i][0], normals[i][1], normals[i][2]);

        if tex_coords.len() > i {
            vertex.tex = vec2(tex_coords[i][0], tex_coords[i][1]);
        }

        if tmp_colors.len() > i {
            vertex.col = vec3(tmp_colors[i][0], tmp_colors[i][1], tmp_colors[i][2]);
        }
        node_mesh.vertices.push(vertex);
    }

    for face in &mesh.faces {
        for index in &face.0 {
            node_mesh.indices.push(*index);
        }
    }
    // for debuging
    println!(
        "num positions {},num normals {}, num tex coords {}, num colors {}",
        mesh.vertices.len(),
        mesh.normals.len(),
        total_texcoords,
        total_colors,
    );

    node_mesh
}
//fn process_node(node: &russimp::node::Node, scene: &russimp::scene::Scene) {}
extern crate collada;
/// helper function to get vertex for collada object
fn get_attributs(obj: &collada::Object, index: &collada::VTNIndex, color: Vec3) -> Vertex {
    let i = index.0;
    let j = index.1.unwrap();
    let k = index.2.unwrap();

    Vertex {
        pos: vec3(
            obj.vertices[i].x as f32,
            obj.vertices[i].y as f32,
            obj.vertices[i].z as f32,
        ),

        norm: vec3(
            obj.normals[k].x as f32,
            obj.normals[k].y as f32,
            obj.normals[k].z as f32,
        ),

        col: color,

        tex: vec2(obj.tex_vertices[j].x as f32, obj.tex_vertices[j].y as f32),
    }
}
pub fn from_dae(path: &Path, color: Vec3) -> Model {
    let doc = collada::document::ColladaDocument::from_path(path).unwrap();
    let mut model = Model::DEFAULT;
    for obj in doc.get_obj_set().unwrap().objects {
        let mut mesh = Mesh::DEFAULT;
        for geometry in &obj.geometry {
            for primitive in &geometry.mesh {
                match primitive {
                    collada::PrimitiveElement::Triangles(triangles) => {
                        for triangle in &triangles.vertices {
                            // not sure about this part but also dont care
                            mesh.indices.push(triangle.0 as u32);
                            mesh.indices.push(triangle.1 as u32);
                            mesh.indices.push(triangle.2 as u32);
                        }
                    }
                    collada::PrimitiveElement::Polylist(polylist) => {
                        for shape in &polylist.shapes {
                            match shape {
                                collada::Shape::Triangle(i, j, k) => {
                                    //first vert
                                    mesh.vertices.push(get_attributs(&obj, &i, color));
                                    //sec vert
                                    mesh.vertices.push(get_attributs(&obj, &j, color));
                                    //third vert
                                    mesh.vertices.push(get_attributs(&obj, &k, color));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        model.meshes.push(mesh);
    }

    model
}
