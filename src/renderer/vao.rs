use super::vertex::Vertex;

use std::mem::offset_of;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct Vao {
    pub vertices: Vec<Vertex>,
    vao: u32,
    vbo: u32,
}

impl Vao {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            vao: 0,
            vbo: 0,
        }
    }

    pub fn create(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::CreateBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            let vert_size = std::mem::size_of::<Vertex>();
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * vert_size) as isize,
                self.vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // _________________________________________________
            // _________________________________________________
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                std::ptr::null(),
            );
            // _________________________________________________
            // _________________________________________________
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                offset_of!(Vertex, norm) as *const c_void,
            );
            // _________________________________________________
            // _________________________________________________
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                vert_size as i32,
                offset_of!(Vertex, tex) as *const c_void,
            );
            // _________________________________________________
            // _________________________________________________
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
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}
