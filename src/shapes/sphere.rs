use crate::math::{vec2::*, vec3::*};
use crate::src::model::*;

pub fn load_sphere(lats: u32, longs: u32) -> Mesh {
    let mut mesh = Mesh::default;

    let lat_angle: f32 = 180.0 / (lats as f32 - 1.0);
    let long_angle: f32 = 360.0 / (longs as f32 - 1.0);
    // tmp vertex
    let mut vert: Vertex = Vertex {
        tc: Vec2::ZERO,
        pos: Vec3::ZERO,
        norm: Vec3::ZERO,
        weights: [0.0, 0.0, 0.0, 0.0],
        bone_ids: [-1, -1, -1, -1],
    };
    // get vertices
    for i in 0..lats {
        let theta = 90.0 - (i as f32) * lat_angle;
        vert.pos.y = theta.to_radians().sin();
        vert.tc.y = i as f32 / (lats as f32 - 1.0);

        let xy: f32 = theta.to_radians().cos();

        for j in 0..longs {
            let alpha: f32 = long_angle * (j as f32);

            vert.pos.x = xy * alpha.to_radians().cos();
            vert.pos.z = xy * alpha.to_radians().sin();

            vert.tc.x = j as f32 / (longs as f32 - 1.0);

            vert.norm = vert.pos;

            mesh.vertices.push(vert.clone());
        }
    }
    //get indices
    for i in 0..(lats - 1) {
        for j in 0..longs {
            mesh.indices.push(i * longs + j);
            mesh.indices.push(i * longs + (j + 1) % longs);
            mesh.indices.push((i + 1) * longs + (j + 1) % longs);

            mesh.indices.push((i + 1) * longs + j);
            mesh.indices.push(i * longs + j);
            mesh.indices.push((i + 1) * longs + (j + 1) % longs);
        }
    }

    mesh
}
