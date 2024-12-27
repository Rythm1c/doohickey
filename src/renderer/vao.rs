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
