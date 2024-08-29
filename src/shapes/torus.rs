use crate::{
    math::{misc::*, vec3::*},
    src::model::*,
};

pub fn torus(divs: u32, col: Vec3) -> Mesh {
    let mut mesh = Mesh::DEFAULT;

    let angle = 360.0 / (divs as f32 - 1.0);

    let mut vertex = Vertex::DEFAULT;
    vertex.col = col;
    // inner radius = 0.3
    // outer radius = 0.7
    for i in 0..divs {
        let epsilon = radians(angle * i as f32);

        for j in 0..divs {
            let theta = radians(angle * j as f32);

            let hyp = 0.7 + 0.3 * theta.cos();

            let x = hyp * epsilon.cos();
            let y = 0.3 * theta.sin();
            let z = hyp * epsilon.sin();

            let nx = theta.cos() * epsilon.cos();
            let ny = theta.sin();
            let nz = theta.cos() * epsilon.sin();

            vertex.pos = vec3(x, y, z);
            vertex.norm = vec3(nx, ny, nz);

            mesh.vertices.push(vertex);
        }
    }

    for i in 0..(divs - 1) {
        for j in 0..divs {
            mesh.indices.push(i * divs + j);
            mesh.indices.push(i * divs + (j + 1) % divs);
            mesh.indices.push((i + 1) * divs + (j + 1) % divs);

            mesh.indices.push((i + 1) * divs + j);
            mesh.indices.push(i * divs + j);
            mesh.indices.push((i + 1) * divs + (j + 1) % divs);
        }
    }

    mesh
}
