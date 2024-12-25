use super::vertex::Vertex;

#[derive(Clone)]
pub struct Buffer<T> {
    pub data: Vec<T>,
    id: u32,
}

pub type EBO = Buffer<u32>;
pub type VBO = Buffer<Vertex>;

impl<T> Default for Buffer<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            id: 0,
        }
    }
}

impl<T> Buffer<T> {
    pub fn create(&mut self) {
        unsafe {
            gl::CreateBuffers(1, &mut self.id);
        }
    }

    pub fn bind(&mut self, target: u32) {
        unsafe {
            gl::BindBuffer(target, self.id);
            gl::BufferData(
                target,
                (self.data.len() * std::mem::size_of::<T>()) as isize,
                self.data.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
