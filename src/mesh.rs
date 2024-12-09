use crate::gl;

use crate::math::{vec2::*, vec3::*};
use std::mem::offset_of;
use std::os::raw::c_void;

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

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    vao: u32,
    vbo: u32,
    ebo: u32,
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

            let vert_size = std::mem::size_of::<Vertex>();
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * vert_size) as isize,
                self.vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            if self.indices.len() != 0 {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                    self.indices.as_ptr() as *const c_void,
                    gl::STATIC_DRAW,
                );
            }

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
                offset_of!(Vertex, norm) as *const c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                offset_of!(Vertex, tex) as *const c_void,
            );

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                offset_of!(Vertex, col) as *const c_void,
            );

            // for animations
            // _________________________________________________
            // _________________________________________________
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                offset_of!(Vertex, weights) as *const c_void,
            );

            gl::EnableVertexAttribArray(5);
            gl::VertexAttribIPointer(
                5,
                4,
                gl::INT,
                vert_size as i32,
                offset_of!(Vertex, bone_ids) as *const c_void,
            );
            // _________________________________________________
            // _________________________________________________

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
