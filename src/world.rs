//use crate::math::quaternion::Quat;
use crate::math::{mat4::*, vec3::*};
use crate::src::{animations, camera, lights, model, physics, shaders, shadows};

use std::collections::HashMap;
use std::ffi::CString;

pub struct World {
    //shaders
    s_obj: shaders::Program, // object shader
    s_shadow: shaders::Program,
    // shaders end
    player: model::Model,
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

        let vert_shader = shaders::Shader::from_vert_src(
            &CString::new(include_str!("../shaders/shader.vert")).unwrap(),
        )
        .unwrap();
        let frag_shader = shaders::Shader::from_frag_src(
            &CString::new(include_str!("../shaders/shader.frag")).unwrap(),
        )
        .unwrap();
        let prgm = shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
        prgm.set_use();
        prgm.update_int("shadowMap", 0);

        let vshader_shadows = shaders::Shader::from_vert_src(
            &CString::new(include_str!("../shaders/shadowmap.vert")).unwrap(),
        )
        .unwrap();
        let fshader_shadows = shaders::Shader::from_frag_src(
            &CString::new(include_str!("../shaders/shadowmap.frag")).unwrap(),
        )
        .unwrap();
        let shadow_program =
            shaders::Program::from_shaders(&[vshader_shadows, fshader_shadows]).unwrap();

        let mut models_: HashMap<String, model::Model> = Default::default();
        let mut ball = model::Model::new(
            model::Shape::Sphere { radius: (1.0) },
            vec3(4.0, 30.0, 10.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        model::load_sphere(50, 50, 4.0, &mut ball);
        ball.checkered = true;
        ball.squares = 20.0;
        models_.insert(String::from("ball"), ball);

        let mut ball2 = model::Model::new(
            model::Shape::Sphere { radius: (1.0) },
            vec3(3.0, 40.0, 10.0),
            vec3(1.0, 0.35, 0.06),
        )
        .unwrap();
        model::load_sphere(50, 50, 3.0, &mut ball2);
        models_.insert(String::from("ball2"), ball2);

        let mut cube = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(1.0, 1.0, 1.0),
            },
            vec3(-12.0, 20.0, 6.0),
            vec3(0.92, 0.29, 0.29),
        )
        .unwrap();
        model::load_cube(Vec3::new(2.0, 2.0, 2.0), &mut cube);
        models_.insert(String::from("cube"), cube);

        let mut cube2 = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(1.0, 1.0, 1.0),
            },
            vec3(5.0, 5.0, 5.0),
            vec3(0.0, 1.0, 0.12),
        )
        .unwrap();
        model::load_cube(Vec3::new(5.0, 5.0, 5.0), &mut cube2);
        models_.insert(String::from("cube2"), cube2);

        let mut platform = model::Model::new(
            model::Shape::Cube {
                dimensions: Vec3::new(1.0, 1.0, 1.0),
            },
            vec3(0.0, -2.0, 0.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        model::load_cube(Vec3::new(1000.0, 2.0, 1000.0), &mut platform);
        platform.sub_dvd = true;
        platform.lines = 70.0;
        models_.insert(String::from("platform"), platform);

        models_
            .values_mut()
            .for_each(|m| m.prepere_render_resources());

        let ls = vec![
            lights::PointLight {
                position: vec3(30.0, 20.0, -20.0),
                color: vec3(1.0, 1.0, 1.0),
            },
            lights::PointLight {
                position: vec3(-30.0, 20.0, -20.0),
                color: vec3(1.0, 0.6, 0.01),
            },
            lights::PointLight {
                position: vec3(30.0, 20.0, 40.0),
                color: vec3(1.0, 0.0, 1.0),
            },
            lights::PointLight {
                position: vec3(-30.0, 20.0, 40.0),
                color: vec3(0.0, 1.0, 0.5),
            },
        ];

        let mut player_ = model::Model::new(
            model::Shape::Cube {
                dimensions: vec3(0.1, 0.1, 0.1),
            },
            vec3(0.0, 2.0, 2.0),
            vec3(1.0, 1.0, 1.0),
        )
        .unwrap();
        //model::from_gltf("man/scene.gltf", &mut player_);
        model::from_dae(std::path::Path::new("Running.dae"), &mut player_);
        player_.prepere_render_resources();

        Ok(World {
            projection: perspective(45.0, ratio, 0.1, 1000.0),
            sun: lights::DirectionalLight {
                shadows: shadows::Shadow::new(1900, 1200),
                color: vec3(1.0, 1.0, 1.0),
                dir: vec3(0.3, -0.7, 0.4),
            },
            s_shadow: shadow_program,
            player: player_,
            models: models_,
            elapsed: 0.0,
            s_obj: prgm,
            lights: ls,
            cam: camera,
        })
    }
    pub fn update_cam(&mut self, ratio: f32) -> &mut World {
        self.projection = perspective(45.0, ratio, 0.1, 1000.0);
        self.cam.update_motion();

        self
    }
    pub fn update_objects(&mut self, dt: f32) -> &mut World {
        self.elapsed += dt;
        animations::spin(
            &mut self
                .models
                .get_mut(&String::from("cube2"))
                .unwrap()
                .rotation,
            dt,
            90.0,
            vec3(0.0, 1.0, 1.0),
        );

        animations::rotate_around(
            &mut self.models.get_mut(&String::from("cube2")).unwrap().pos,
            vec3(0.0, 20.0, 20.0),
            30.0,
            45.0,
            vec3(0.0, 1.0, 0.0),
            self.elapsed,
        );

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
        self.player.update_properties();

        for model in self.models.values_mut() {
            model.update_properties();
        }

        self
    }
    pub fn add_shadows(&mut self) -> &mut Self {
        self.sun.shadows.attach(1900, 1200);

        self.s_shadow.set_use();
        self.s_shadow.update_mat4(
            "lightSpace",
            self.sun.get_projection() * self.sun.get_view(),
        );

        self.s_shadow.update_mat4("model", self.player.transform);
        self.player.render();

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
            self.s_obj.update_vec3(
                format!("pointLights[{}].position", i).as_str(),
                self.lights[i].position,
            );
            self.s_obj.update_vec3(
                format!("pointLights[{}].color", i).as_str(),
                self.lights[i].color,
            );
        }

        self.s_obj.update_mat4("transform", self.player.transform);
        self.s_obj
            .update_int("textured", self.player.textured as i32);
        self.s_obj.update_vec3("col", self.player.color);
        self.s_obj
            .update_int("checkered", self.player.checkered as i32);
        self.s_obj.update_float("squares", self.player.squares);
        self.s_obj
            .update_int("subDivided", self.player.sub_dvd as i32);
        self.s_obj.update_float("lines", self.player.lines);
        self.player.render();

        // object specific
        self.models.values_mut().for_each(|model| {
            self.s_obj.update_mat4("transform", model.transform);
            self.s_obj.update_int("textured", model.textured as i32);
            self.s_obj.update_vec3("col", model.color);
            self.s_obj.update_int("checkered", model.checkered as i32);
            self.s_obj.update_float("squares", model.squares);
            self.s_obj.update_int("subDivided", model.sub_dvd as i32);
            self.s_obj.update_float("lines", model.lines);

            model.render();
        })
    }
}
