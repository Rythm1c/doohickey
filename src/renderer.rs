use super::shaders::Program;

pub trait Renderable {
    fn render(&self, program: &mut Program);
}
