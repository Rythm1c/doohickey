use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;

pub fn load_cube(color_cube: bool, color: Vec3) -> Mesh {
    let mut mesh = Mesh::DEFAULT;
    //front face
    for v in &DATA {
        mesh.vertices.push(Vertex {
            col: color,
            pos: vec3(v[0], v[1], v[2]),
            norm: vec3(v[3], v[4], v[5]),
            tex: vec2(v[6], v[7]),
        })
    }

    if color_cube {
        for face_col in 0..6 {
            mesh.vertices[face_col * 4 + 0].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 4 + 1].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 4 + 2].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 4 + 3].col = FACE_COLORS[face_col];
        }
    }

    mesh.indices = Vec::from(&INDICES);

    mesh
}
/// work in progress...
pub fn cube_sphere(divs: u32, color_cube: bool, color: Vec3) -> Mesh {
    let mut mesh = Mesh::DEFAULT;

    for point in &INDICES {
        let p = *point as usize;
        let mut p1 = Vertex {
            col: color,
            pos: vec3(DATA[p][0], DATA[p][1], DATA[p][2]),
            norm: vec3(DATA[p][3], DATA[p][4], DATA[p][5]),
            tex: vec2(DATA[p][6], DATA[p][7]),
        };
        p1.pos = project_to_sphere(p1.pos);

        mesh.vertices.push(p1);
    }

    if color_cube {
        for face_col in 0..6 {
            mesh.vertices[face_col * 6 + 0].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 6 + 1].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 6 + 2].col = FACE_COLORS[face_col];

            mesh.vertices[face_col * 6 + 4].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 6 + 4].col = FACE_COLORS[face_col];
            mesh.vertices[face_col * 6 + 5].col = FACE_COLORS[face_col];
        }
    }

    for _ in 0..divs {
        let mut final_mesh = Mesh::DEFAULT;
        let range = 0..(mesh.vertices.len() / 3);
        for face in range {
            let v1 = mesh.vertices[face * 3 + 0];
            let v2 = mesh.vertices[face * 3 + 1];
            let v3 = mesh.vertices[face * 3 + 2];

            let p1 = divide(v1, v2);
            let p2 = divide(v1, v3);
            let p3 = divide(v2, v3);

            add_tri(&mut final_mesh, v1, p1, p2);
            add_tri(&mut final_mesh, p1, v2, p3);
            add_tri(&mut final_mesh, p1, p2, p3);
            add_tri(&mut final_mesh, p2, p3, v3);
        }
        mesh = final_mesh;
    }

    mesh
}
fn divide(v1: Vertex, v2: Vertex) -> Vertex {
    let mut v3 = Vertex {
        pos: Vec3::ZERO,
        norm: Vec3::ZERO,
        tex: Vec2::ZERO,
        col: Vec3::ONE,
    };

    v3.pos.x = v1.pos.x + v2.pos.x;
    v3.pos.y = v1.pos.y + v2.pos.y;
    v3.pos.z = v1.pos.z + v2.pos.z;

    v3.pos = project_to_sphere(v3.pos);

    v3.col = (v1.col + v2.col) / 2.0;

    v3.norm = v3.pos;
    v3.tex = vec2((v1.tex.x + v2.tex.x) / 2.0, (v1.tex.y + v2.tex.y) / 2.0);

    v3
}
/// project to a unit sphere
fn project_to_sphere(v: Vec3) -> Vec3 {
    let scale = 1.0 / (v.x.powf(2.0) + v.y.powf(2.0) + v.z.powf(2.0)).sqrt();

    v * scale
}

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
