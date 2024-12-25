use super::vertex::Vertex;

use std::mem::offset_of;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn create(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn set_attributes() {
        let vert_size = std::mem::size_of::<Vertex>();
        unsafe {
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
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}
