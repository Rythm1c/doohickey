use crate::src::math::{vec2::*, vec3::*};

use std::mem::offset_of;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Vertex {
    pub pos: [f32; 3],
    pub norm: [f32; 3],
    pub tex: [f32; 2],
    pub col: [f32; 3],

    pub weights: [f32; 4],
    pub bone_ids: [i32; 4],
}
impl Vertex {
    pub const DEFAULT: Self = Self {
        pos: [0.0; 3],
        norm: [0.0; 3],
        tex: [0.0; 2],
        col: [0.0; 3],

        weights: [0.0; 4],
        bone_ids: [-1; 4],
    };

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
}
