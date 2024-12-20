#[derive(Clone)]
pub struct Ebo {
    pub indices: Vec<u32>,
    ebo: u32,
}

impl Ebo {
    pub fn new() -> Self {
        Self {
            indices: (Vec::new()),
            ebo: (0),
        }
    }

    pub fn create(&mut self) {
        unsafe {
            gl::CreateBuffers(1, &mut self.ebo);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
        }
    }
}
impl Drop for Ebo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.ebo);
        }
    }
}
