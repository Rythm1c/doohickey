use crate::src::renderer::buffer::EBO;
use crate::src::renderer::mesh::Mesh;
use crate::src::renderer::vertex::Vertex;

pub fn sphere(lats: u32, longs: u32, color: [f32; 3]) -> Mesh {
    let mut mesh = Mesh::default();

    let lat_angle: f32 = 180.0 / (lats as f32 - 1.0);
    let long_angle: f32 = 360.0 / (longs as f32 - 1.0);
    // tmp vertex
    let mut vert: Vertex = Vertex::DEFAULT;
    vert.col = color;

    // get vertices
    for i in 0..lats {
        let theta = 90.0 - (i as f32) * lat_angle;
        vert.pos[1] = theta.to_radians().sin();
        vert.tex[1] = i as f32 / (lats as f32 - 1.0);

        let xy: f32 = theta.to_radians().cos();

        for j in 0..longs {
            let alpha: f32 = long_angle * (j as f32);

            vert.pos[0] = xy * alpha.to_radians().cos();
            vert.pos[2] = xy * alpha.to_radians().sin();

            vert.tex[0] = j as f32 / (longs as f32 - 1.0);

            vert.norm = vert.pos;

            mesh.vbo.data.push(vert.clone());
        }
    }

    //get indices
    mesh.ebo = Some(EBO::default());
    let indices = &mut mesh.ebo.as_mut().unwrap().data;
    for i in 0..(lats - 1) {
        for j in 0..longs {
            indices.push(i * longs + j);
            indices.push(i * longs + (j + 1) % longs);
            indices.push((i + 1) * longs + (j + 1) % longs);

            indices.push((i + 1) * longs + j);
            indices.push(i * longs + j);
            indices.push((i + 1) * longs + (j + 1) % longs);
        }
    }
    mesh.create();

    mesh
}
