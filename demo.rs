use super::src::engine::input;
use super::src::engine::timer::Timer;
use super::src::engine::window::Window;
use super::src::scene::world::World;
use super::src::screen_capture::system::ScreenCapture;

pub struct Demo {
    window: Window,
    world: World,
    timer: Timer,
    running: bool,
    screen_capture: ScreenCapture,
}

impl Demo {
    pub fn new() -> Self {
        let window = Window::create(String::from("rust engine"), 800, 600);
        let world = World::new(window.get_ratio());
        let timer = Timer::new();

        let (w, h) = window.get_size();
        let screen_capture = ScreenCapture::new(w, h);

        let running = true;

        Self {
            running,
            window,
            world,
            timer,
            screen_capture,
        }
    }

    pub fn run(&mut self) {
        self.init().main_loop().close();
    }

    fn init(&mut self) -> &mut Self {
        self
    }

    fn main_loop(&mut self) -> &mut Self {
        while self.running {
            self.timer.update();
            self.handle_input();
            self.update();
        }

        self
    }

    fn update(&mut self) {
        self.world
            .update_cam(self.window.get_ratio())
            .update_animations(&self.timer)
            .update_physics()
            .update_shadows();

        self.window.clear();

        self.world.render();

        //recorder.screen_shot("screenshot.png");
        self.screen_capture.capture();

        eprint!("\rfps : {}", self.timer.fps());

        self.window.swap();
    }

    fn handle_input(&mut self) {
        for event in self.window.event_pump.poll_iter() {
            if matches!(event, sdl2::event::Event::Quit { .. }) {
                self.running = false;
            }

            input::mouse_input(&event, &mut self.world.camera);
        }
    }

    fn close(&mut self) {
        // use std::path::Path;
        // self.screen_capture.save_video(&Path::new("test.mp4"));
        eprintln!("\nDone");
    }
}
