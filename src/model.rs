extern crate gltf;
extern crate tobj;

use crate::gl;

use crate::math::{mat4::*, quaternion::*, vec2::*, vec3::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pos: Vec3,
    norm: Vec3,
    tc: Vec2,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere, /* { radius: f32 } */
    Cube,   /* { scale: Vec3 } */
    Quad,
    Other,
}

struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: u32,
    vbo: u32,
    ebo: u32,
    pub shape: Shape,
}
pub struct Model {
    //containers for render data
    meshes: Vec<Mesh>,
    pub color: Vec3,
    pub transform: Mat4,
    pub size: Vec3,
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
    pub fn new(_pos: Vec3, _size: Vec3, col: Vec3) -> Result<Model, String> {
        let t = mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Ok(Model {
            meshes: vec![],
            transform: t,
            color: col,
            velocity: vec3(0.0, 0.0, 0.0),
            pos: _pos,
            size: _size,
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
        self.transform = mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        self.transform = self.transform * translate(&self.pos);
        self.transform = self.transform * rotate(self.rotation.s, self.rotation.axis());
        self.transform = self.transform * scale(&self.size);
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

pub fn load_sphere(lats: u32, longs: u32, model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
        shape: Shape::Sphere,
    };
    let lat_angle: f32 = 180.0 / (lats as f32 - 1.0);
    let long_angle: f32 = 360.0 / (longs as f32 - 1.0);
    // tmp vertex
    let mut vert: Vertex = Vertex {
        pos: vec3(0.0, 0.0, 0.0),
        norm: vec3(0.0, 0.0, 0.0),
        tc: vec2(0.0, 0.0),
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
pub fn load_quad(model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
        shape: Shape::Quad,
    };

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
}
pub fn load_cube(model: &mut Model) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![],
        vao: 0,
        vbo: 0,
        ebo: 0,
        shape: Shape::Cube,
    };

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
