use gl;
use sdl2;

pub struct Window {
    handle: sdl2::video::Window,
    _context: sdl2::video::GLContext,
    pub event_pump: sdl2::EventPump,
}

impl Window {
    pub fn create(title: String, w: u32, h: u32) -> Self {
        let sdl = sdl2::init().unwrap();

        let video_sub_sys = sdl.video().unwrap();

        let gl_attr = video_sub_sys.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let handle = video_sub_sys
            .window(&title[..], w, h)
            .position(100, 100)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let _context = handle.gl_create_context().unwrap();
        use std::os::raw::c_void;
        let _gl = gl::load_with(|s| video_sub_sys.gl_get_proc_address(s) as *const c_void);

        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::Enable(gl::DEPTH_TEST);
        }

        video_sub_sys.gl_set_swap_interval(1).unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Self {
            handle,
            _context,
            event_pump,
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.handle.size()
    }

    pub fn get_ratio(&self) -> f32 {
        let (w, h) = self.get_size();

        w as f32 / h as f32
    }

    pub fn clear(&self, r: f32, g: f32, b: f32) {
        let (w, h) = self.get_size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn swap(&self) {
        self.handle.gl_swap_window();
    }
}
