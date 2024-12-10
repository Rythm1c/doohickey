use crate::gl;
use crate::math::{mat4::*, vec3::*};

use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Program {
    id: gl::types::GLuint,
}

/// function assumes there will only be a vertex and fragment shader  
/// no geometry shader capabilities for this engine yet and not planning on adding anytime soon
pub fn create_shader(vert: &Path, frag: &Path) -> Program {
    Program::from_shaders(&[
        Shader::from_vert_src(&vert).unwrap(),
        Shader::from_frag_src(&frag).unwrap(),
    ])
    .unwrap()
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }
        unsafe { gl::LinkProgram(program_id) };

        let mut success: gl::types::GLint = 1;

        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn update_vec3(&self, name: &str, vec: Vec3) {
        unsafe {
            let n = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, n.as_ptr());
            gl::Uniform3f(location, vec.x, vec.y, vec.z);
        }
    }
    pub fn update_mat4(&self, name: &str, mat: Mat4) {
        unsafe {
            let n = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, n.as_ptr());
            gl::UniformMatrix4fv(location, 1, true as u8, &mat.data[0][0]);
        }
    }
    pub fn update_int(&self, name: &str, value: i32) {
        unsafe {
            let n = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, n.as_ptr());
            gl::Uniform1i(location, value);
        }
    }
    pub fn update_float(&self, name: &str, value: f32) {
        unsafe {
            let n = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, n.as_ptr());
            gl::Uniform1f(location, value);
        }
    }
    pub fn set_use(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_src(src: &Path, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_src(src, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_src(src: &Path) -> Result<Shader, String> {
        Shader::from_src(src, gl::VERTEX_SHADER)
    }
    pub fn from_frag_src(src: &Path) -> Result<Shader, String> {
        Shader::from_src(src, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_src(path: &Path, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let mut file = File::open(path).unwrap();

    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();

    let src_as_cstr = CString::new(src).unwrap();

    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &src_as_cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { CString::from_vec_unchecked(buffer) }
}
