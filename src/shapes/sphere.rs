use crate::math::{misc::*, vec3::*};
use crate::src::model::*;
use crate::src::shapes::patterns::*;

pub fn load_sphere(lats: u32, longs: u32, color: Vec3, pattern: Option<Pattern>) -> Mesh {
    let mut mesh = Mesh::default();

    let lat_angle: f32 = 180.0 / (lats as f32 - 1.0);
    let long_angle: f32 = 360.0 / (longs as f32 - 1.0);
    // tmp vertex
    let mut vert: Vertex = Vertex::DEFAULT;

    // get vertices
    for i in 0..lats {
        let theta = 90.0 - (i as f32) * lat_angle;
        vert.pos.y = theta.to_radians().sin();
        vert.tex.y = i as f32 / (lats as f32 - 1.0);

        let xy: f32 = theta.to_radians().cos();

        for j in 0..longs {
            let alpha: f32 = long_angle * (j as f32);

            vert.pos.x = xy * alpha.to_radians().cos();
            vert.pos.z = xy * alpha.to_radians().sin();

            vert.tex.x = j as f32 / (longs as f32 - 1.0);
            // for shading
            if let Some(choice) = pattern {
                let s = shaded(vert.tex, choice);
                vert.col = s * color;
            } else {
                vert.col = color;
            }

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

pub fn load_icosphere(divs: i32, color: Vec3) -> Mesh {
    let lat_angle = (0.5 as f32).atan();
    let long_angle = radians(72.0);
    let mut tmp = Mesh::default();

    let mut vertex = Vertex::DEFAULT;
    vertex.col = color;

    //first vertex
    vertex.pos = vec3(0.0, 1.0, 0.0);
    vertex.norm = vertex.pos;
    tmp.vertices.push(vertex);

    let mut y = lat_angle.sin();
    let mut hyp = lat_angle.cos();
    for j in 0..5 {
        let x = hyp * ((j as f32) * long_angle).cos();
        let z = hyp * ((j as f32) * long_angle).sin();

        vertex.pos = vec3(x, y, z);
        vertex.norm = vertex.pos;
        tmp.vertices.push(vertex);
    }

    y = (-lat_angle).sin();
    hyp = (-lat_angle).cos();
    for j in 0..5 {
        let x = hyp * ((j as f32) * long_angle + (long_angle / 2.0)).cos();
        let z = hyp * ((j as f32) * long_angle + (long_angle / 2.0)).sin();

        vertex.pos = vec3(x, y, z);
        vertex.norm = vertex.pos;
        tmp.vertices.push(vertex);
    }

    vertex.pos = vec3(0.0, -1.0, 0.0);
    vertex.norm = vertex.pos;
    tmp.vertices.push(vertex);
    // arranges verts into triangles
    let mut mesh = Mesh::default();

    let triangles: Vec<[usize; 3]> = vec![
        [0, 1, 2],
        [0, 2, 3],
        [0, 3, 4],
        [0, 4, 5],
        [0, 5, 1],
        [1, 6, 2],
        [2, 6, 7],
        [2, 7, 3],
        [3, 7, 8],
        [3, 8, 4],
        [4, 8, 9],
        [4, 9, 5],
        [5, 9, 10],
        [5, 10, 1],
        [1, 10, 6],
        [11, 6, 7],
        [11, 7, 8],
        [11, 8, 9],
        [11, 9, 10],
        [11, 10, 6],
    ];
    for triangle in triangles {
        add_tri(
            &mut mesh,
            tmp.vertices[triangle[0]],
            tmp.vertices[triangle[1]],
            tmp.vertices[triangle[2]],
        );
    }

    for _ in 0..divs {
        let mut final_mesh = Mesh::default();
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
    let mut v3 = Vertex::DEFAULT;

    v3.pos.x = v1.pos.x + v2.pos.x;
    v3.pos.y = v1.pos.y + v2.pos.y;
    v3.pos.z = v1.pos.z + v2.pos.z;

    v3.pos = project_to_sphere(v3.pos);

    v3.col = (v1.col + v2.col) / 2.0;

    v3.norm = v3.pos;

    v3
}
/// work in progress...  
/// thinking of a way to arange those triangles is a pain in the ass
#[allow(dead_code)]
pub fn dodecahedron(divs: i32, color: Vec3) -> Mesh {
    let icosphere = load_icosphere(divs, color);
    let mut mesh = Mesh::default();
    //assuming its a triangle
    let faces = 0..(icosphere.vertices.len() / 3);

    for face in faces {
        let verts = [
            icosphere.vertices[face * 3 + 0],
            icosphere.vertices[face * 3 + 1],
            icosphere.vertices[face * 3 + 2],
        ];
        mesh.vertices.push(centroid(verts));
    }

    mesh
}
#[allow(dead_code)]
fn centroid(face: [Vertex; 3]) -> Vertex {
    let mut average = Vertex::DEFAULT;

    for vert in face {
        average.pos = average.pos + vert.pos;
        average.col = average.col + vert.col;
    }
    average.pos = project_to_sphere(average.pos / 3.0);
    average.col = average.col / 3.0;
    average.norm = average.pos;

    average
}

fn project_to_sphere(v: Vec3) -> Vec3 {
    let scale = 1.0 / (v.x.powf(2.0) + v.y.powf(2.0) + v.z.powf(2.0)).sqrt();

    v * scale
}
