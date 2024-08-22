//use crate::math::quaternion::Quat;

use crate::math::{mat4::*, vec3::*};
use crate::src::shapes::{cube::*, sphere::*};
use crate::src::{animations, camera, lights, model, physics, player, shaders, shadows};

use std::collections::HashMap;
//use std::ffi::CString;
use std::path::Path;
pub struct World {
    //shaders
    s_obj: shaders::Program, // object shader
    s_shadow: shaders::Program,
    // shaders end
    player: player::Player,
    pub cam: camera::Camera,
    pub models: HashMap<String, model::Model>,
    projection: Mat4,
    sun: lights::DirectionalLight,
    lights: Vec<lights::PointLight>,
    elapsed: f32,
}

impl World {
    pub fn new(ratio: f32) -> Result<World, String> {
        let camera = camera::Camera::new(
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 5.0, -30.0),
            0.5,
        )
        .unwrap();
        use shaders::{Program, Shader};
        let vert_shader = Shader::from_vert_src(Path::new("shaders/shader.vert")).unwrap();
        let frag_shader = Shader::from_frag_src(Path::new("shaders/shader.frag")).unwrap();
        let prgm = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        let vshader_shadows = Shader::from_vert_src(Path::new("shaders/shadowmap.vert")).unwrap();
        let fshader_shadows = Shader::from_frag_src(Path::new("shaders/shadowmap.frag")).unwrap();
        let shadow_program = Program::from_shaders(&[vshader_shadows, fshader_shadows]).unwrap();

        let mut models_: HashMap<String, model::Model> = Default::default();
        let mut ball = model::Model::new(
            model::Shape::Sphere { radius: (4.0) },
            vec3(4.0, 30.0, 10.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        ball.meshes.push(load_sphere(100, 100));
        ball.checkered = true;
        ball.squares = 20.0;
        models_.insert(String::from("ball"), ball);

        let mut ball2 = model::Model::new(
            model::Shape::Sphere { radius: (3.0) },
            vec3(3.0, 40.0, 10.0),
            vec3(1.0, 0.35, 0.06),
        )
        .unwrap();
        ball2.meshes.push(load_sphere(100, 100));
        models_.insert(String::from("ball2"), ball2);

        let mut cube = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(2.0, 2.0, 2.0),
            },
            vec3(-12.0, 20.0, 6.0),
            vec3(0.92, 0.29, 0.29),
        )
        .unwrap();
        cube.meshes.push(load_cube());
        models_.insert(String::from("cube"), cube);

        let mut cube2 = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(5.0, 5.0, 5.0),
            },
            vec3(5.0, 5.0, 5.0),
            vec3(0.0, 1.0, 0.12),
        )
        .unwrap();
        cube2.meshes.push(load_cube());
        models_.insert(String::from("cube2"), cube2);

        let mut platform = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(1000.0, 2.0, 1000.0),
            },
            vec3(0.0, -2.0, 0.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        platform.meshes.push(load_cube());
        platform.sub_dvd = true;
        platform.lines = 70.0;
        models_.insert(String::from("platform"), platform);

        models_
            .values_mut()
            .for_each(|m| m.prepere_render_resources());

        let ls = vec![
            lights::PointLight {
                pos: vec3(30.0, 20.0, -20.0),
                col: vec3(1.0, 1.0, 1.0),
            },
            lights::PointLight {
                pos: vec3(-30.0, 20.0, -20.0),
                col: vec3(1.0, 0.6, 0.01),
            },
            lights::PointLight {
                pos: vec3(30.0, 20.0, 40.0),
                col: vec3(1.0, 0.0, 1.0),
            },
            lights::PointLight {
                pos: vec3(-30.0, 20.0, 40.0),
                col: vec3(0.0, 1.0, 0.5),
            },
        ];

        let mut player_ = player::Player::new(&String::from("mannequin"));
        player_.model = model::from_dae(Path::new("Running.dae"));
        player_.model.shape = model::Shape::Cube {
            dimensions: Vec3::new(0.1, 0.1, 0.1),
        };

        player_.model.prepere_render_resources();

        Ok(World {
            projection: perspective(45.0, ratio, 0.1, 1000.0),
            s_shadow: shadow_program,
            player: player_,
            elapsed: 0.0,
            s_obj: prgm,
            models: models_,
            lights: ls,
            cam: camera,
            sun: lights::DirectionalLight {
                shadows: shadows::Shadow::new(1900, 1200),
                color: vec3(1.0, 1.0, 1.0),
                dir: vec3(0.3, -0.7, 0.4),
            },
        })
    }
    pub fn update_cam(&mut self, ratio: f32) -> &mut Self {
        self.projection = perspective(45.0, ratio, 0.1, 1000.0);
        self.cam.update_motion();

        self
    }
    pub fn update_animations(&mut self, dt: f32) -> &mut Self {
        self.elapsed += dt;

        animations::spin(
            dt,
            90.0,
            vec3(0.0, 1.0, 1.0),
            &mut self
                .models
                .get_mut(&String::from("cube2"))
                .unwrap()
                .rotation,
        );

        animations::rotate_around(
            vec3(0.0, 20.0, 20.0),
            30.0,
            45.0,
            vec3(0.0, 1.0, 0.0),
            self.elapsed,
            &mut self.models.get_mut(&String::from("cube2")).unwrap().pos,
        );

        self
    }
    pub fn update_physics(&mut self) -> &mut Self {
        physics::collision_sphere_sphere(
            String::from("ball"),
            String::from("ball2"),
            &mut self.models,
        );
        physics::collision_sphere_aabb(
            String::from("ball"),
            String::from("platform"),
            &mut self.models,
        );
        physics::collision_sphere_aabb(
            String::from("ball2"),
            String::from("platform"),
            &mut self.models,
        );
        physics::collision_aabb_aabb(
            String::from("cube"),
            String::from("platform"),
            &mut self.models,
        );

        physics::gravity(&mut self.models.get_mut(&String::from("cube")).unwrap().velocity);
        physics::gravity(&mut self.models.get_mut(&String::from("ball")).unwrap().velocity);
        physics::gravity(
            &mut self
                .models
                .get_mut(&String::from("ball2"))
                .unwrap()
                .velocity,
        );

        self
    }
    pub fn update_objects(&mut self) -> &mut Self {
        self.player.model.update_properties();

        for model in self.models.values_mut() {
            model.update_properties();
        }

        self
    }
    pub fn update_shadows(&mut self) -> &mut Self {
        self.sun.shadows.attach(1900, 1200);

        self.s_shadow.set_use();
        self.s_shadow.update_mat4(
            "lightSpace",
            self.sun.get_projection() * self.sun.get_view(),
        );

        self.s_shadow
            .update_mat4("model", self.player.model.transform);
        self.player.model.render();

        self.models.values_mut().for_each(|model| {
            self.s_shadow.update_mat4("model", model.transform);
            model.render();
        });
        // end of render
        shadows::Shadow::detach();
        self
    }

    pub fn render(&mut self) {
        self.s_obj.set_use();
        self.sun.shadows.bind_texture();
        self.s_obj.update_vec3("L_direction", self.sun.dir);
        self.s_obj.update_vec3("L_color", self.sun.color);
        self.s_obj.update_vec3("viewPos", self.cam.pos);
        self.s_obj.update_mat4("view", self.cam.get_view_mat());
        self.s_obj.update_mat4("projection", self.projection);
        self.s_obj.update_mat4(
            "lightSpace",
            self.sun.get_projection() * self.sun.get_view(),
        );
        self.s_obj.update_int("shadowsEnabled", true as i32);

        self.s_obj
            .update_int("pointLightCount", self.lights.len() as i32);
        // update point lights
        for i in 0..(self.lights.len()) {
            pl_to_shader(&self.lights[i], &mut self.s_obj, i);
        }

        model_to_shader(&mut self.player.model, &mut self.s_obj);
        self.player.model.render();

        // object specific
        self.models.values_mut().for_each(|model| {
            model_to_shader(model, &mut self.s_obj);
            model.render();
        })
    }
}
/// send point light to shaders point light array
fn pl_to_shader(light: &lights::PointLight, shader: &mut shaders::Program, i: usize) {
    shader.update_vec3(format!("pointLights[{}].position", i).as_str(), light.pos);
    shader.update_vec3(format!("pointLights[{}].color", i).as_str(), light.col);
}
// send player info to shader for drawing
fn model_to_shader(model: &mut model::Model, shader: &mut shaders::Program) {
    /* shader.update_mat4("boneTransform", Mat4::new()); */
    shader.update_mat4("transform", model.transform);
    shader.update_int("textured", model.textured as i32);
    shader.update_vec3("col", model.color);
    shader.update_int("checkered", model.checkered as i32);
    shader.update_float("squares", model.squares);
    shader.update_int("subDivided", model.sub_dvd as i32);
    shader.update_float("lines", model.lines);
}
