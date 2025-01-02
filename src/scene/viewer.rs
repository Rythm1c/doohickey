use std::collections::HashMap;
use std::path::Path;

use super::camera::Camera;
use super::lights::*;
use crate::src::foreign::*;

use crate::src::renderer::{model::*, shaders, shadows};
use shaders::Program;

use crate::src::engine::timer::Timer;
use crate::src::math::{quaternion::Quat, vec3::*};

// abit messy but who cares
// not sure why im bothering with comments as if anyone is going to read any of this
pub struct World {
    pub camera: Camera,
    pub player: Model,
    pub sun: DirectionalLight,
    shaders: HashMap<String, Program>, //done
    pub point_lights: Vec<PointLight>, //done
}

impl World {
    pub fn default() -> Self {
        Self {
            camera: Camera::default(),
            player: Model::default(),
            sun: DirectionalLight::default(),
            shaders: HashMap::new(),
            point_lights: Vec::new(),
        }
    }

    pub fn new() -> Self {
        let mut camera = Camera::default();
        camera.pos = vec3(0.0, 20.0, -30.0);

        let mut shaders = HashMap::new();
        let mut point_lights = Vec::new();

        let sun = DirectionalLight {
            shadows: shadows::Shadow::new(1900, 1200),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };

        let file = gltf::Gltf::new(Path::new("models/astronaut"));
        let mut player = Model::default();
        file.populate_model(&mut player);
        player.translate(vec3(0.0, 12.0, 3.0));
        player.scale(vec3(0.5, 0.5, 0.5));
        player.orient(Quat::create(180.0, vec3(0.0, 1.0, 0.0)));

        player.play_animation = true;
        player.current_anim = 0;

        let phong = shaders::create_shader(
            &Path::new("shaders/common.vert"),
            &Path::new("shaders/phong.frag"),
        );
        let phong_anim = shaders::create_shader(
            &Path::new("shaders/animation.vert"),
            &Path::new("shaders/phong.frag"),
        );

        let pbr = shaders::create_shader(
            &Path::new("shaders/common.vert"),
            &Path::new("shaders/pbr.frag"),
        );
        let pbr_anim = shaders::create_shader(
            &Path::new("shaders/animation.vert"),
            &Path::new("shaders/pbr.frag"),
        );

        shaders.insert("phong".to_string(), phong);
        shaders.insert("phongAnimation".to_string(), phong_anim);
        shaders.insert("pbr".to_string(), pbr);
        shaders.insert("pbrAnimation".to_string(), pbr_anim);

        // prepare the textures in the shaders for rendering
        {
            let obj_shader = shaders.get_mut("phong").unwrap();

            obj_shader.set_use();
            obj_shader.update_int("shadowMap", 0);
            obj_shader.update_int("albedo", 1);
            obj_shader.update_int("specular", 2);
        }

        {
            let anim_shader = shaders.get_mut("phongAnimation").unwrap();

            anim_shader.set_use();
            anim_shader.update_int("shadowMap", 0);
            anim_shader.update_int("albedo", 1);
            anim_shader.update_int("specular", 2);
        }

        Self {
            sun,
            camera,
            player,
            point_lights,
            shaders,
        }
    }

    pub fn update(&mut self, win_ratio: f32, timer: &Timer) {
        // update camera movement
        self.camera.update_motion();
        // update animations for current model being viewed
        self.player.update_animation(timer.elapsed);

        let lights = &self.point_lights;
        //________________________________________________________________________
        //update shader for static objects(no skeleton)
        {
            let shader = &mut self.shaders.get_mut("phong").unwrap();

            shader.set_use();
            self.sun.shadows.bind_texture();
            shader.update_vec3("L_direction", self.sun.dir);
            shader.update_vec3("L_color", self.sun.color);
            shader.update_vec3("viewPos", self.camera.pos);
            shader.update_mat4("view", &self.camera.get_view());

            shader.update_mat4("lightSpace", &self.sun.transform());
            shader.update_int("shadowsEnabled", false as i32);

            let projection = self.camera.get_pojection(win_ratio);
            shader.update_mat4("projection", &projection);

            let len = lights.len();
            shader.update_int("pointLightCount", len as i32);

            for i in 0..len {
                pl_to_shader(lights[i], shader, i);
            }
        }
        //________________________________________________________________________
        //update shader for dynamic objects(have a skeleton)
        {
            let shader = &mut self.shaders.get_mut("phongAnimation").unwrap();
            let projection = self.camera.get_pojection(win_ratio);

            shader.set_use();
            self.sun.shadows.bind_texture();
            shader.update_vec3("L_direction", self.sun.dir);
            shader.update_vec3("L_color", self.sun.color);
            shader.update_vec3("viewPos", self.camera.pos);
            shader.update_mat4("view", &self.camera.get_view());

            shader.update_mat4("projection", &projection);
            shader.update_mat4("lightSpace", &self.sun.transform());
            shader.update_int("shadowsEnabled", false as i32);

            let len = lights.len();

            shader.update_int("pointLightCount", len as i32);
            // update point lights
            for i in 0..len {
                pl_to_shader(lights[i], shader, i);
            }
        }
    }

    pub fn render(&mut self) {
        // let shader = &mut self.shaders.get_mut("phong").unwrap();
        //render scene shadows
        let shader = &mut self.shaders.get_mut("shadow").unwrap();
        self.sun.shadows.attach(1900, 1200);
        shader.set_use();
        shader.update_mat4("lightSpace", &self.sun.transform());
        self.player.render(shader);
        shadows::Shadow::detach();

        //render model animated
        let shader = &mut self.shaders.get_mut("phongAnimation").unwrap();
        shader.set_use();

        self.player.render(shader);
    }
}
