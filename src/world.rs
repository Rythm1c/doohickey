use crate::math::{mat4::*, vec3::*};
use crate::src::shapes::{cube::load_cube, sphere::*, torus::torus};
use crate::src::{
    animations, assets::Assets, camera, foreign::*, lights, model::*, object::*, physics, shaders,
    shadows,
};

use std::path::Path;
pub struct World {
    pub camera: camera::Camera,
    s_obj: shaders::Program,
    s_shadow: shaders::Program,
    player: Object,
    projection: Mat4,
    sun: lights::DirectionalLight,
    assets: Assets,
    elapsed: f32,
}

impl World {
    pub fn new(ratio: f32) -> Result<World, String> {
        let camera = camera::Camera::new(
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 5.0, 0.0),
            vec3(0.0, 20.0, -30.0),
            0.5,
        )
        .unwrap();

        let s_obj = create_shader(
            Path::new("shaders/shader.vert"),
            Path::new("shaders/shader.frag"),
        );

        let s_shadow = create_shader(
            Path::new("shaders/shadowmap.vert"),
            Path::new("shaders/shadowmap.frag"),
        );

        let mut assets = Assets::new();
        let mut model = Model::DEFAULT;

        model.add_mesh(load_sphere(100, 100, vec3(1.0, 1.0, 1.0)));
        model.squares = 20.0;
        model.checkered = true;
        let mut object = Object::new();

        object
            .change_pos(vec3(4.0, 30.0, 10.0))
            .change_shape(Shape::Sphere { radius: (4.0) })
            .update_model(model);
        assets.add_object("ball", object.clone());

        model = Model::DEFAULT;
        model.add_mesh(load_icosphere(3, vec3(1.0, 0.35, 0.06)));
        object
            .change_pos(vec3(15.0, 40.0, 10.0))
            .change_shape(Shape::Sphere { radius: (7.0) })
            .update_model(model);
        assets.add_object("ball2", object.clone());

        model = Model::DEFAULT;
        model.add_mesh(load_cube(false, vec3(1.0, 0.13, 0.48)));
        object
            .change_pos(vec3(-15.0, 40.0, 20.0))
            .change_shape(Shape::Cube {
                dimensions: vec3(6.0, 6.0, 6.0),
            })
            .update_model(model);
        assets.add_object("cube", object.clone());

        model = Model::DEFAULT;
        model.add_mesh(load_cube(true, Vec3::ZERO));
        object
            .change_pos(vec3(5.0, 5.0, 5.0))
            .change_shape(Shape::Cube {
                dimensions: Vec3::new(5.0, 5.0, 5.0),
            })
            .update_model(model);
        assets.add_object("cube2", object.clone());

        model = Model::DEFAULT;
        model.add_mesh(torus(50, vec3(0.64, 1.0, 0.13)));
        object
            .change_pos(vec3(-15.0, 5.0, -5.0))
            .change_shape(Shape::Cube {
                dimensions: Vec3::new(10.0, 10.0, 10.0),
            })
            .update_model(model);
        assets.add_object("torus", object.clone());

        model = Model::DEFAULT;
        model.add_mesh(load_cube(false, vec3(0.9, 0.9, 0.9)));
        model.sub_dvd = true;
        model.lines = 70.0;
        object
            .change_pos(vec3(0.0, -2.0, 0.0))
            .change_shape(Shape::Cube {
                dimensions: Vec3::new(1000.0, 2.0, 1000.0),
            })
            .update_model(model.clone());
        assets.add_object("platform", object);

        assets.objects.values_mut().for_each(|object| {
            object.model.prepere_render_resources();
        });

        assets.add_pointlight(lights::PointLight {
            pos: vec3(30.0, 20.0, -20.0),
            col: vec3(1.0, 1.0, 1.0),
        });

        assets.add_pointlight(lights::PointLight {
            pos: vec3(-30.0, 20.0, -20.0),
            col: vec3(1.0, 0.6, 0.01),
        });

        assets.add_pointlight(lights::PointLight {
            pos: vec3(30.0, 20.0, 40.0),
            col: vec3(1.0, 0.0, 1.0),
        });
        assets.add_pointlight(lights::PointLight {
            pos: vec3(-30.0, 20.0, 40.0),
            col: vec3(0.0, 1.0, 0.5),
        });

        let mut player = Object::new();
        player
            .change_pos(vec3(0.0, 12.0, 3.0))
            .change_shape(Shape::Cube {
                dimensions: vec3(0.1, 0.1, 0.1),
            })
            .update_model(from_extern_src(Path::new("Running.fbx")));
        player.model.prepere_render_resources();

        let sun = lights::DirectionalLight {
            shadows: shadows::Shadow::new(1900, 1200),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };
        //from_dae(Path::new("Running.dae"), vec3(1.0, 1.0, 1.0))

        let projection = perspective(45.0, ratio, 0.1, 1000.0);

        Ok(World {
            sun,
            camera,
            s_shadow,
            player,
            s_obj,
            assets,
            elapsed: 0.0,
            projection,
        })
    }
    pub fn update_cam(&mut self, ratio: f32) -> &mut Self {
        self.projection = perspective(45.0, ratio, 0.1, 1000.0);
        self.camera.update_motion();

        self
    }
    pub fn update_animations(&mut self, dt: f32) -> &mut Self {
        self.elapsed += dt;

        animations::spin(
            dt,
            90.0,
            vec3(0.0, 1.0, 1.0),
            &mut self.assets.get_object("cube2").transform,
        );

        animations::rotate_around(
            vec3(0.0, 20.0, 20.0),
            50.0,
            22.5,
            vec3(0.0, 1.0, 0.0),
            self.elapsed,
            &mut self.assets.get_object("cube2").transform.pos,
        );

        self
    }
    pub fn update_physics(&mut self) -> &mut Self {
        physics::collision_sphere_sphere(
            String::from("ball"),
            String::from("ball2"),
            &mut self.assets.objects,
        );
        physics::collision_sphere_aabb(
            String::from("ball"),
            String::from("platform"),
            &mut self.assets.objects,
        );
        physics::collision_sphere_aabb(
            String::from("ball2"),
            String::from("platform"),
            &mut self.assets.objects,
        );
        physics::collision_aabb_aabb(
            String::from("cube"),
            String::from("platform"),
            &mut self.assets.objects,
        );

        physics::gravity(&mut self.assets.get_object("cube").transform.velocity);
        physics::gravity(&mut self.assets.get_object("ball").transform.velocity);
        physics::gravity(&mut self.assets.get_object("ball2").transform.velocity);

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
            .update_mat4("model", self.player.transform.get());
        self.player.model.render();

        self.assets.objects.values_mut().for_each(|object| {
            self.s_shadow.update_mat4("model", object.transform.get());
            object.model.render();
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
        self.s_obj.update_vec3("viewPos", self.camera.pos);
        self.s_obj.update_mat4("view", self.camera.get_view_mat());
        self.s_obj.update_mat4("projection", self.projection);
        self.s_obj.update_mat4(
            "lightSpace",
            self.sun.get_projection() * self.sun.get_view(),
        );
        self.s_obj.update_int("shadowsEnabled", false as i32);

        self.s_obj
            .update_int("pointLightCount", self.assets.point_lights.len() as i32);
        // update point lights
        for i in 0..(self.assets.point_lights.len()) {
            pl_to_shader(&self.assets.point_lights[i], &mut self.s_obj, i);
        }

        model_to_shader(&mut self.player, &mut self.s_obj);
        self.player.model.render();

        // object specific
        self.assets.objects.values_mut().for_each(|object| {
            model_to_shader(object, &mut self.s_obj);
            object.model.render();
        })
    }
}
/// send point light to shaders point light array
fn pl_to_shader(light: &lights::PointLight, shader: &mut shaders::Program, i: usize) {
    shader.update_vec3(format!("pointLights[{}].position", i).as_str(), light.pos);
    shader.update_vec3(format!("pointLights[{}].color", i).as_str(), light.col);
}
// send player info to shader for drawing
fn model_to_shader(o: &mut Object, shader: &mut shaders::Program) {
    /* shader.update_mat4("boneTransform", Mat4::new()); */
    shader.update_mat4("transform", o.transform.get());
    shader.update_int("textured", o.model.textured as i32);
    shader.update_int("checkered", o.model.checkered as i32);
    shader.update_float("squares", o.model.squares);
    shader.update_int("subDivided", o.model.sub_dvd as i32);
    shader.update_float("lines", o.model.lines);
}

use shaders::{Program, Shader};
fn create_shader(vert: &Path, frag: &Path) -> Program {
    Program::from_shaders(&[
        Shader::from_vert_src(&vert).unwrap(),
        Shader::from_frag_src(&frag).unwrap(),
    ])
    .unwrap()
}
