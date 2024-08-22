use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;

pub fn load_cube() -> Mesh {
    let mut mesh = Mesh::default;
    //front face
    let mut tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    //back face
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    //left face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    //right face
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    //top face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    //bottom face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(0.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(1.0, 0.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(0.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(1.0, 1.0),
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    mesh.vertices.push(tmp_vertex);

    mesh.indices = vec![
        0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 18,
        19, 19, 17, 16, 20, 22, 23, 23, 21, 20,
    ];

    mesh
}
