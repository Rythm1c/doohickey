use crate::src::math::transform::Transform;
use crate::src::math::vec3::Vec3;
use crate::src::renderer::mesh::Mesh;
use crate::src::renderer::shaders::Program;

pub struct Shape {
    mesh: Mesh,
    pub transform: Transform,
    pattern: Option<Pattern>,
    pub velocity: Vec3,
}
/// choose what pattern to give a shape

#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    /// darkness, number of squares
    Checkered(f32, i32),

    /// darkness, thickness, number of lines
    Striped(f32, f32, i32),
}
/*
pub trait Shape {
    fn change_orientation(&mut self, new_orienation: &Quat) -> &mut Self;
    fn change_pos(&mut self, new_pos: Vec3) -> &mut Self;
    fn reshape(&mut self, mesh: Mesh) -> &mut Self;
    fn add_velocity(&mut self);
    fn render(&self, program: &shaders::Program);
} */

impl Shape {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            transform: Transform::DEFAULT,
            pattern: None,
            velocity: Vec3::ZERO,
        }
    }

    pub fn reposition(&mut self, new_pos: Vec3) -> &mut Self {
        self.transform.translation = new_pos;
        self
    }

    pub fn rescale(&mut self, new_size: Vec3) -> &mut Self {
        self.transform.scaling = new_size;
        self
    }

    pub fn change_pattern(&mut self, pattern: Pattern) -> &mut Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn reshape(&mut self, mesh: Mesh) -> &mut Self {
        self.mesh = mesh;
        self
    }

    pub fn add_velocity(&mut self) {
        self.transform.translation = self.transform.translation + self.velocity;
    }

    pub fn create(&mut self) {
        self.mesh.create();
    }

    pub fn render(&mut self, shader: &mut Program) {
        shader.update_mat4("transform", &self.transform.to_mat());
        /*    shader.update_int("textured", o.model.textured as i32); */
        if let Some(pattern) = self.pattern {
            match pattern {
                // 1 : true
                // 0 : false
                Pattern::Checkered(a, b) => {
                    shader.update_int("checkered", 1);
                    shader.update_float("sqr_shade", a);
                    shader.update_float("squares", b as f32);
                    shader.update_int("subDivided", 0);
                }
                Pattern::Striped(a, b, c) => {
                    shader.update_int("subDivided", 1);
                    shader.update_float("line_shade", a);
                    shader.update_float("line_thickness", b);
                    shader.update_float("lines", c as f32);
                    shader.update_int("checkered", 0);
                }
            }
        } else {
            shader.update_int("checkered", 0);
            shader.update_int("subDivided", 0);
        }

        self.mesh.render();
    }
}
