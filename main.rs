use src::{input, world};

extern crate gl;
extern crate sdl2;
use std::time::Instant;

mod math;
mod src;

fn main() {
    let sdl = sdl2::init().unwrap();

    let video_sub_sys = sdl.video().unwrap();

    let gl_attr = video_sub_sys.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let mut w: i32 = 1200;
    let mut h: i32 = 800;

    let window = video_sub_sys
        .window("rust engine", w as u32, h as u32)
        .position(100, 100)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_sub_sys.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, w, h);
        gl::Enable(gl::DEPTH_TEST);
    }

    // video_sub_sys.gl_set_swap_interval(1);

    let mut event_pump = sdl.event_pump().unwrap();

    let mut world = world::World::new(w as f32, h as f32).unwrap();

    let mut running = true;
    let mut delta = 0.0;

    while running {
        let now = Instant::now();

        for event in event_pump.poll_iter() {
            input::window_input(&event, &mut running, &mut w, &mut h);
            input::mouse_input(&event, &mut world.cam);
        }
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 1200, 800);
        }
        world
            .update_cam(w as f32, h as f32)
            .update_objects(delta)
            .render_shadows()
            .render();

        window.gl_swap_window();

        delta = now.elapsed().as_secs_f32();
        println!("fps : {}", (1.0 / delta));
    }
}
