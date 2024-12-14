use gl;
use sdl2;

mod src;

use src::engine::input;
use src::engine::timer::Timer;
use src::engine::window::Window;
use src::scene::world::World;
use src::screen_capture::system::ScreenCapture;

fn main() {
    let mut win = Window::create(String::from("rust engine"), 800, 600);

    let mut world = World::new(win.get_ratio());

    let mut timer = Timer::new();

    let (w, h) = win.get_size();
    let mut recorder = ScreenCapture::new(w, h);

    'mainloop: loop {
        timer.update();

        for event in win.event_pump.poll_iter() {
            if matches!(event, sdl2::event::Event::Quit { .. }) {
                break 'mainloop;
            }

            input::mouse_input(&event, &mut world.camera);
        }

        world
            .update_cam(win.get_ratio())
            .update_animations(&timer)
            .update_physics()
            .update_shadows();

        win.clear();

        world.render();
        world.render_skeletal_animations();

        //recorder.capture();

        eprint!("\rfps : {}", timer.fps());

        win.swap();
    }

    eprintln!("");

    // use std::path::Path;
    // recorder.save_video(&Path::new("test.mp4"));
    eprintln!("Done");
}
