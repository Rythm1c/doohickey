use crate::gl;
use crate::gl::types::*;
use crate::math::mat4::Mat4;
use crate::math::{vec2::*, vec3::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub pos: Vec3,
    pub norm: Vec3,
    pub tex: Vec2,
    pub col: Vec3,

    pub weights: [f32; 4],
    pub bone_ids: [i32; 4],
}
impl Vertex {
    pub const DEFAULT: Self = Self {
        pos: Vec3::ZERO,
        norm: Vec3::ZERO,
        tex: Vec2::ZERO,
        col: Vec3::ONE,

        weights: [0.0; 4],
        bone_ids: [-1; 4],
    };
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
#[derive(Clone)]
pub struct BoneInfo {
    pub name: String,
    pub parent: usize,
    pub bind_pose: Mat4,
}

#[derive(Clone)]
pub struct Mesh {
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
    pub skeleton: Vec<BoneInfo>,
}
impl Mesh {
    pub fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    pub fn create(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::CreateBuffers(1, &mut self.vbo);
            gl::CreateBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            let float_size = std::mem::size_of::<f32>();
            let vert_size = std::mem::size_of::<Vertex>();
            //let int_size = std::mem::size_of::<i32>();

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

            // for animations
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                (11 * float_size) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(5);
            gl::VertexAttribIPointer(
                5,
                4,
                gl::INT,
                vert_size as i32,
                (15 * float_size) as *const GLvoid,
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
    pub fn default() -> Self {
        Self {
            meshes: Vec::new(),
            textured: false,
            checkered: false,
            squares: 0.0,
            sub_dvd: false,
            lines: 0.0,

            skeleton: Vec::new(),
        }
    }

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

        weights: p1.weights,
        bone_ids: p1.bone_ids,
    });
    mesh.vertices.push(Vertex {
        pos: p2.pos,
        norm: normal,
        tex: p2.tex,
        col: p2.col,

        weights: p2.weights,
        bone_ids: p2.bone_ids,
    });
    mesh.vertices.push(Vertex {
        pos: p3.pos,
        norm: normal,
        tex: p3.tex,
        col: p3.col,

        weights: p3.weights,
        bone_ids: p3.bone_ids,
    });
}

//use crate::src::player::{Bone, Player};
