//extern crate tobj;
use crate::gl;

use crate::math::{mat4::*, quaternion::*, vec2::*, vec3::*};

//const MAX_BONE_INFLUENCE: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pos: Vec3,
    norm: Vec3,
    tc: Vec2,
    //weights: [f32; MAX_BONE_INFLUENCE],
    //bone_ids: [i32; MAX_BONE_INFLUENCE],
}

/// mostly for collision detection
/// specify the most appropriate shape to determine the bounding volume for collisions
#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere { radius: f32 },
    Cube { dimensions: Vec3 },
    /*  Quad,
    Other, */
}

#[derive(PartialEq, Clone)]
struct Mesh {
    //containers for render data
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ebo: u32,
}
pub struct Model {
    meshes: Vec<Mesh>,
    pub shape: Shape,
    pub color: Vec3,
    pub transform: Mat4,
    pub pos: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub textured: bool,
    pub checkered: bool,
    pub squares: f32,
    pub sub_dvd: bool,
    pub lines: f32,
}
impl Mesh {
    pub fn create(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::CreateBuffers(1, &mut self.vbo);
            gl::CreateBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                self.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as gl::types::GLsizei,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn render(&mut self) {
        if self.indices.len() != 0 {
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices.len().try_into().unwrap(),
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        } else {
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
                gl::BindVertexArray(0);
            }
        }
    }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteBuffers(1, &mut self.ebo);
        }
    }
}
impl Model {
    pub fn new(_shape: Shape, _pos: Vec3, col: Vec3) -> Result<Model, String> {
        let t = mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Ok(Model {
            meshes: vec![],
            transform: t,
            color: col,
            velocity: vec3(0.0, 0.0, 0.0),
            pos: _pos,
            shape: _shape,
            rotation: quat(0.0, 0.0, 0.0, 0.0),
            textured: false,
            checkered: false,
            squares: 0.0,
            sub_dvd: false,
            lines: 0.0,
        })
    }

    pub fn update_properties(&mut self) {
        self.pos = self.pos + self.velocity;
        self.transform = Mat4::new();
        self.transform = self.transform * translate(&self.pos);
        self.transform = self.transform * rotate(self.rotation.s, self.rotation.axis());

        match self.shape {
            Shape::Cube { dimensions } => {
                self.transform = self.transform * scale(&dimensions);
            }
            Shape::Sphere { radius } => {
                self.transform = self.transform * scale(&vec3(radius, radius, radius));
            } //_ => {}
        }
    }

    pub fn prepere_render_resources(&mut self) {
        for mesh in self.meshes.iter_mut() {
            mesh.create();
        }
    }
    pub fn render(&mut self) {
        for mesh in self.meshes.iter_mut() {
            mesh.render();
        }
    }
}

use std::path::Path;
extern crate collada;
pub fn from_dae(path: &Path, model: &mut Model) {
    let doc = collada::document::ColladaDocument::from_path(path).unwrap();

    for obj in doc.get_obj_set().unwrap().objects {
        let mut mesh = Mesh {
            vertices: vec![],
            indices: vec![],
            vao: 0,
            vbo: 0,
            ebo: 0,
        };
        for geometry in obj.geometry {
            for primitive in geometry.mesh {
                match primitive {
                    collada::PrimitiveElement::Triangles(triangles) => {
                        println!("mesh triangles {}", triangles.vertices.len());

                        for triangle in triangles.vertices {
                            mesh.indices.push(triangle.0 as u32);
                            mesh.indices.push(triangle.1 as u32);
                            mesh.indices.push(triangle.2 as u32);
                        }
                    }
                    collada::PrimitiveElement::Polylist(polylist) => {
                        for shape in polylist.shapes {
                            match shape {
                                collada::Shape::Triangle(i, j, k) => {
                                    let mut vertex = Vertex {
                                        pos: Vec3::ZERO,
                                        norm: Vec3::ZERO,
                                        tc: Vec2::ZERO,
                                    };
                                    vertex.pos = vec3(
                                        obj.vertices[i.0].x as f32,
                                        obj.vertices[i.0].y as f32,
                                        obj.vertices[i.0].z as f32,
                                    );
                                    vertex.tc = vec2(
                                        obj.tex_vertices[i.1.unwrap()].x as f32,
                                        obj.tex_vertices[i.1.unwrap()].y as f32,
                                    );
                                    vertex.norm = vec3(
                                        obj.normals[i.2.unwrap()].x as f32,
                                        obj.normals[i.2.unwrap()].y as f32,
                                        obj.normals[i.2.unwrap()].z as f32,
                                    );
                                    mesh.vertices.push(vertex);

                                    //sec vert
                                    vertex.pos = vec3(
                                        obj.vertices[j.0].x as f32,
                                        obj.vertices[j.0].y as f32,
                                        obj.vertices[j.0].z as f32,
                                    );
                                    vertex.tc = vec2(
                                        obj.tex_vertices[j.1.unwrap()].x as f32,
                                        obj.tex_vertices[j.1.unwrap()].y as f32,
                                    );
                                    vertex.norm = vec3(
                                        obj.normals[j.2.unwrap()].x as f32,
                                        obj.normals[j.2.unwrap()].y as f32,
                                        obj.normals[j.2.unwrap()].z as f32,
                                    );
                                    mesh.vertices.push(vertex);

                                    //third vert
                                    vertex.pos = vec3(
                                        obj.vertices[k.0].x as f32,
                                        obj.vertices[k.0].y as f32,
                                        obj.vertices[k.0].z as f32,
                                    );
                                    vertex.tc = vec2(
                                        obj.tex_vertices[k.1.unwrap()].x as f32,
                                        obj.tex_vertices[k.1.unwrap()].y as f32,
                                    );
                                    vertex.norm = vec3(
                                        obj.normals[k.2.unwrap()].x as f32,
                                        obj.normals[k.2.unwrap()].y as f32,
                                        obj.normals[k.2.unwrap()].z as f32,
                                    );
                                    mesh.vertices.push(vertex);
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
}

extern crate gltf;
#[allow(dead_code)]
pub fn from_gltf(path: &str, model: &mut Model) {
    let (document, buffers, ..) = gltf::import(path).unwrap();

    for mesh in document.meshes() {
        //prepare for next batch of data
        let mut tmp_mesh = Mesh {
            vertices: vec![],
            indices: vec![],
            vao: 0,
            vbo: 0,
            ebo: 0,
        };

        let primitives = mesh.primitives();
        primitives.for_each(|primitive| {
            // let indices = primitive.indices();
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            //temporary array to hold position data
            let mut tmp_positions: Vec<Vec3> = vec![];
            // extract positions
            if let Some(positions) = reader.read_positions()
            /* .map(|v| dbg!(v)) */
            {
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
            if let Some(normals) = reader.read_normals()
            /* .map(|n| dbg!(n)) */
            {
                for norm in normals {
                    tmp_normals.push(Vec3 {
                        x: norm[0],
                        y: norm[1],
                        z: norm[2],
                    })
                }
            }
            //temporary storage for texure coordinates
            let mut tmp_tex_coords: Vec<Vec2> = vec![];
            //extract
            if let Some(gltf::mesh::util::ReadTexCoords::F32(gltf::accessor::Iter::Standard(itr))) =
                reader.read_tex_coords(0)
            /* .map(|tc| dbg!(tc)) */
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
            /* .map(|i| dbg!(i)) */
            {
                for index in itr {
                    tmp_mesh.indices.push(index);
                }
            }

            for i in 0..tmp_positions.len() {
                tmp_mesh.vertices.push(Vertex {
                    norm: tmp_normals[i],
                    pos: tmp_positions[i],
                    tc: tmp_tex_coords[i],
                })
            }
            model.meshes.push(tmp_mesh.clone());
        })
    }
}

pub fn load_sphere(lats: u32, longs: u32, r: f32, model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
    };
    model.shape = Shape::Sphere { radius: r };
    let lat_angle: f32 = 180.0 / (lats as f32 - 1.0);
    let long_angle: f32 = 360.0 / (longs as f32 - 1.0);
    // tmp vertex
    let mut vert: Vertex = Vertex {
        tc: Vec2::ZERO,
        pos: Vec3::ZERO,
        norm: Vec3::ZERO,
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

    model.meshes.push(mesh);
}
/* pub fn load_quad(model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
    };

    model.shape = Shape::Quad;

    let mut vertex: Vertex = Vertex {
        tc: vec2(0.0, 0.0),
        pos: vec3(-1.0, -1.0, 0.0),
        norm: vec3(0.0, 0.0, 0.0),
    };
    mesh.vertices.push(vertex);
    vertex = Vertex {
        tc: vec2(0.0, 1.0),
        pos: vec3(-1.0, 1.0, 0.0),
        norm: vec3(0.0, 0.0, 0.0),
    };
    mesh.vertices.push(vertex);
    vertex = Vertex {
        tc: vec2(1.0, 1.0),
        pos: vec3(1.0, 1.0, 0.0),
        norm: vec3(0.0, 0.0, 0.0),
    };
    mesh.vertices.push(vertex);
    vertex = Vertex {
        tc: vec2(1.0, 0.0),
        pos: vec3(1.0, -1.0, 0.0),
        norm: vec3(0.0, 0.0, 0.0),
    };
    mesh.vertices.push(vertex);

    mesh.indices = vec![0, 1, 2, 0, 2, 3];
    model.meshes.push(mesh);
} */
pub fn load_cube(size: Vec3, model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
    };
    model.shape = Shape::Cube { dimensions: size };
    //front face
    let mut tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(0.0, 0.0, 1.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    //back face
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(0.0, 0.0, -1.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    //left face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(-1.0, 0.0, 0.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    //right face
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(1.0, 0.0, 0.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    //top face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, 1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, 1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, 1.0, -1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, 1.0, -1.0),
        norm: vec3(0.0, 1.0, 0.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    //bottom face
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, 1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(0.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, 1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(1.0, 0.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(-1.0, -1.0, -1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(0.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);
    tmp_vertex = Vertex {
        pos: vec3(1.0, -1.0, -1.0),
        norm: vec3(0.0, -1.0, 0.0),
        tc: vec2(1.0, 1.0),
    };
    mesh.vertices.push(tmp_vertex);

    mesh.indices = vec![
        0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 18,
        19, 19, 17, 16, 20, 22, 23, 23, 21, 20,
    ];
    model.meshes.push(mesh);
}
