//extern crate tobj;
use crate::gl;
use crate::gl::types::*;
use crate::math::{vec2::*, vec3::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub pos: Vec3,
    pub norm: Vec3,
    pub tex: Vec2,
    pub col: Vec3,
}

/// mostly for collision detection
/// specify the most appropriate shape to determine the bounding volume for collisions
#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere { radius: f32 },
    Cube { dimensions: Vec3 },
    None,
    /*  Quad, */
}

#[derive(PartialEq, Clone)]
pub struct Mesh {
    //containers for render data
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ebo: u32,
}

#[derive(Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textured: bool,
    pub checkered: bool,
    pub squares: f32,
    pub sub_dvd: bool,
    pub lines: f32,
}
impl Mesh {
    pub const DEFAULT: Self = Self {
        vertices: Vec::new(),
        indices: Vec::new(),
        vao: 0,
        vbo: 0,
        ebo: 0,
    };

    pub fn create(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::CreateBuffers(1, &mut self.vbo);
            gl::CreateBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            let float_size = std::mem::size_of::<f32>();
            let vert_size = std::mem::size_of::<Vertex>();

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * vert_size) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                (3 * float_size) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                (6 * float_size) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                (8 * float_size) as *const GLvoid,
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
    pub const DEFAULT: Self = Self {
        meshes: Vec::new(),
        textured: false,
        checkered: false,
        squares: 0.0,
        sub_dvd: false,
        lines: 0.0,
    };

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
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
pub fn add_tri(mesh: &mut Mesh, p1: Vertex, p2: Vertex, p3: Vertex) {
    let normal = (p1.norm + p2.norm + p3.norm) / 3.0;

    mesh.vertices.push(Vertex {
        pos: p1.pos,
        norm: normal,
        tex: p1.tex,
        col: p1.col,
    });
    mesh.vertices.push(Vertex {
        pos: p2.pos,
        norm: normal,
        tex: p2.tex,
        col: p2.col,
    });
    mesh.vertices.push(Vertex {
        pos: p3.pos,
        norm: normal,
        tex: p3.tex,
        col: p3.col,
    });
}

//use crate::src::player::{Bone, Player};
use std::path::Path;

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
                                    //first
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
            let  mut tmp_colors: Vec<Vec3> = vec![];
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
