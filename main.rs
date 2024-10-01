extern crate gl;
extern crate sdl2;

mod math;
mod screen_capture;
mod src;

use screen_capture::system::ScreenCapture;
use src::{
    input::{self, WinInfo},
    timer, world,
};

fn main() {
    let sdl = sdl2::init().unwrap();

    let video_sub_sys = sdl.video().unwrap();

    let gl_attr = video_sub_sys.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let mut win_info = WinInfo {
        running: true,
        w: 1200,
        h: 800,
    };

    let window = video_sub_sys
        .window("rust engine", win_info.w as u32, win_info.h as u32)
        .position(100, 100)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    use std::os::raw::c_void;
    let _gl = gl::load_with(|s| video_sub_sys.gl_get_proc_address(s) as *const c_void);

    video_sub_sys.gl_set_swap_interval(1).unwrap();

    unsafe {
        gl::Viewport(0, 0, win_info.w, win_info.h);
        gl::Enable(gl::DEPTH_TEST);
    }

    video_sub_sys.gl_set_swap_interval(1).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut world = world::World::new(win_info.get_ratio());

    let mut timer = timer::Timer::new();

    let mut screen_shot = ScreenCapture::new(win_info.w as u32, win_info.h as u32);

    while win_info.running {
        timer.update();

        for event in event_pump.poll_iter() {
            input::window_input(&event, &mut win_info);
            input::mouse_input(&event, &mut world.camera);
        }

        world
            .update_cam(win_info.get_ratio())
            .update_animations(&timer)
            .update_physics()
            .update_shadows();

        unsafe {
            gl::Viewport(0, 0, win_info.w, win_info.h);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        world.render();
        world.render_skeletal_animations();

        screen_shot.capture();

        window.gl_swap_window();

        let fps = 1.0 / timer.delta;
        eprint!("\rfps : {fps}");
    }

    screen_shot.save_video(&String::from("frame"));

    eprintln!("\nFinished");
}
