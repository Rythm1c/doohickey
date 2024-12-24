use crate::src::math::{vec2::*, vec3::*};
use crate::src::renderer::buffer::EBO;
use crate::src::renderer::mesh::Mesh;
use crate::src::renderer::vertex::Vertex;

pub fn cube(color_cube: bool, color: Vec3) -> Mesh {
    let mut mesh = Mesh::default();

    //front face
    for v in &DATA {
        mesh.vbo.data.push(Vertex {
            col: color,
            pos: vec3(v[0], v[1], v[2]),
            norm: vec3(v[3], v[4], v[5]),
            tex: vec2(v[6], v[7]),

            bone_ids: [-1; 4],
            weights: [0.0; 4],
        })
    }

    if color_cube {
        for i in 0..6 {
            for j in 0..4 {
                mesh.vbo.data[i * 4 + j].col = FACE_COLORS[i];
            }
        }
    }

    mesh.ebo = Some(EBO::default());
    mesh.ebo.as_mut().unwrap().data = Vec::from(&INDICES);

    mesh
}

/// work in progress...

const FACE_COLORS: [Vec3; 6] = [
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 1.0,
    },
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    },
    Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0,
    },
];

const DATA: [[f32; 8]; 24] = [
    [1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0],
    [-1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [-1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    //
    [1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0],
    [-1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0],
    [-1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0],
    [1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0],
    //
    [-1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
    [-1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0],
    [-1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0],
    [-1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0],
    // right face
    [1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0],
    [1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0],
    // top face
    [-1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    [-1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0],
    [1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0],
    // bottom face
    [-1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0],
    [1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0],
    [-1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0],
    [1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0],
];
const INDICES: [u32; 36] = [
    0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 18, 19,
    19, 17, 16, 20, 22, 23, 23, 21, 20,
];
