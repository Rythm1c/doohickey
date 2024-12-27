use super::src::engine::input;
use super::src::engine::timer::Timer;
use super::src::engine::window::Window;
use super::src::scene::lights;
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
        let world = World::new();
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
        // prepare the textures in the shaders for rendering
        {
            let obj_shader = self.world.shaders.get_mut("phong").unwrap();

            obj_shader.set_use();
            obj_shader.update_int("shadowMap", 0);
            obj_shader.update_int("albedo", 1);
            obj_shader.update_int("specular", 2);
        }

        {
            let anim_shader = self.world.shaders.get_mut("phongAnimation").unwrap();

            anim_shader.set_use();
            anim_shader.update_int("shadowMap", 0);
            anim_shader.update_int("albedo", 1);
            anim_shader.update_int("specular", 2);
        }

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
            .update_cam()
            .update_animations(&self.timer)
            .update_physics()
            .update_shadows();

        self.window.clear();

        let lights = &self.world.point_lights;
        //________________________________________________________________________
        //update shader for static objects(no skeleton)
        {
            let shader = &mut self.world.shaders.get_mut("phong").unwrap();

            shader.set_use();
            self.world.sun.shadows.bind_texture();
            shader.update_vec3("L_direction", self.world.sun.dir);
            shader.update_vec3("L_color", self.world.sun.color);
            shader.update_vec3("viewPos", self.world.camera.pos);
            shader.update_mat4("view", &self.world.camera.get_view());
            let projection = self.world.camera.get_pojection(self.window.get_ratio());
            shader.update_mat4("projection", &projection);
            shader.update_mat4("lightSpace", &self.world.sun.transform());
            shader.update_int("shadowsEnabled", false as i32);
            let len = lights.len();
            shader.update_int("pointLightCount", len as i32);

            for i in 0..len {
                lights::pl_to_shader(lights[i], shader, i);
            }
        }
        //________________________________________________________________________
        //update shader for dynamic objects(have a skeleton)
        {
            let shader = &mut self.world.shaders.get_mut("phongAnimation").unwrap();
            let projection = self.world.camera.get_pojection(self.window.get_ratio());

            shader.set_use();
            self.world.sun.shadows.bind_texture();
            shader.update_vec3("L_direction", self.world.sun.dir);
            shader.update_vec3("L_color", self.world.sun.color);
            shader.update_vec3("viewPos", self.world.camera.pos);
            shader.update_mat4("view", &self.world.camera.get_view());

            shader.update_mat4("projection", &projection);
            shader.update_mat4("lightSpace", &self.world.sun.transform());
            shader.update_int("shadowsEnabled", false as i32);

            let len = lights.len();

            shader.update_int("pointLightCount", len as i32);
            // update point lights
            for i in 0..len {
                lights::pl_to_shader(lights[i], shader, i);
            }
        }

        self.world.render();
        // recorder.screen_shot("screenshot.png");
        // self.screen_capture.capture();

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
