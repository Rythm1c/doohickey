use std::collections::HashMap;
use std::path::Path;

use super::animation::*;
use super::camera::Camera;
use super::foreign::*;
use super::lights;
use super::lights::PointLight;
use super::model::Model;

use super::shaders;
use super::shadows;

use super::timer::Timer;
use crate::math::{mat4::*, quaternion::Quat, vec3::*};

use crate::physics::collision;
use crate::physics::force;

use super::shapes::{cube::cube, shape::Pattern, shape::Shape, sphere::*, torus::torus};

// abit messy but who cares
// not sure why im bothering with comments as if anyone is going to read any of this
pub struct World {
    pub camera: Camera,
    player: Model,
    projection: Mat4,
    sun: lights::DirectionalLight,
    shapes: HashMap<String, Shape>,
    shaders: HashMap<String, Program>,
    point_lights: Vec<PointLight>,
}

impl World {
    pub fn new(ratio: f32) -> Self {
        let mut camera = Camera::default();
        camera.pos = vec3(0.0, 20.0, -30.0);

        let s_obj = create_shader(
            Path::new("shaders/shader.vert"),
            Path::new("shaders/shader.frag"),
        );

        let s_shadow = create_shader(
            Path::new("shaders/shadowmap.vert"),
            Path::new("shaders/shadowmap.frag"),
        );

        let s_animation = create_shader(
            Path::new("shaders/animation.vert"),
            Path::new("shaders/shader.frag"),
        );
        let mut shaders = HashMap::new();

        shaders.insert(String::from("object"), s_obj);
        shaders.insert(String::from("shadow"), s_shadow);
        shaders.insert(String::from("animation"), s_animation);

        let mut shapes = HashMap::new();
        let mut shape = Shape::new();
        shape
            .reshape(sphere(200, 200, vec3(1.0, 1.0, 1.0)))
            .reposition(vec3(4.0, 30.0, 10.0))
            .rescale(vec3(4.0, 4.0, 4.0))
            .change_pattern(Pattern::Checkered(0.3, 20));
        shapes.insert(String::from("ball"), shape);

        let mut shape = Shape::new();
        shape
            .reshape(icosphere(4, vec3(1.0, 0.35, 0.06)))
            .reposition(vec3(15.0, 40.0, 10.0))
            .rescale(vec3(7.0, 7.0, 7.0));
        shapes.insert(String::from("ball2"), shape);

        let mut shape = Shape::new();
        shape
            .reshape(cube(false, vec3(1.0, 0.13, 0.48)))
            .reposition(vec3(-15.0, 40.0, 20.0))
            .rescale(vec3(6.0, 6.0, 6.0));
        shapes.insert(String::from("cube"), shape);

        let mut shape = Shape::new();
        shape
            .reshape(cube(true, Vec3::ZERO))
            .reposition(vec3(5.0, 5.0, 5.0))
            .rescale(vec3(5.0, 5.0, 5.0));
        shapes.insert(String::from("cube2"), shape);

        let mut shape = Shape::new();
        shape
            .reshape(torus(50, vec3(0.64, 1.0, 0.13)))
            .reposition(vec3(-15.0, 5.0, -5.0))
            .rescale(vec3(10.0, 10.0, 10.0));
        shapes.insert(String::from("torus"), shape);

        let mut shape = Shape::new();
        shape
            .reshape(cube(false, vec3(0.9, 0.9, 0.9)))
            .reposition(vec3(0.0, -2.0, 0.0))
            .rescale(vec3(1e3, 2.0, 1e3))
            .change_pattern(Pattern::Striped(0.1, 5e-3, 70));
        shapes.insert(String::from("platform"), shape);

        shapes.values_mut().for_each(|shape| {
            shape.create();
        });

        let mut point_lights = Vec::new();

        point_lights.push(lights::PointLight {
            pos: vec3(30.0, 20.0, -20.0),
            col: vec3(1.0, 1.0, 1.0),
        });

        point_lights.push(lights::PointLight {
            pos: vec3(-30.0, 20.0, -20.0),
            col: vec3(1.0, 0.6, 0.01),
        });

        point_lights.push(lights::PointLight {
            pos: vec3(30.0, 20.0, 40.0),
            col: vec3(1.0, 0.0, 1.0),
        });
        point_lights.push(lights::PointLight {
            pos: vec3(-30.0, 20.0, 40.0),
            col: vec3(0.0, 1.0, 0.5),
        });

        let sun = lights::DirectionalLight {
            shadows: shadows::Shadow::new(1900, 1200),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };

        let mut player = Model::default();
        let file = gltf::GltfFile::new(Path::new("models/alien/Alien.gltf"));

        player.meshes = file.extract_meshes();
        player.skeleton.rest_pose = file.extract_rest_pose();
        player.skeleton.inverse_bind_pose = file.extract_inverse_bind_mats();
        player.skeleton.joint_names = file.extract_joint_names();
        player.animations = file.extract_animations();
        player
            .change_pos(vec3(0.0, 12.0, 3.0))
            .change_size(vec3(3.5, 3.5, 3.5));

        player.prepere_render_resources();
        player.transform.orientation = Quat::create(180.0, vec3(0.0, 1.0, 0.0));
        player.play_animation = true;
        player.current_anim = 2;

        let projection = perspective(45.0, ratio, 0.1, 1e3);

        Self {
            shapes,
            sun,
            camera,
            player,
            point_lights,
            shaders,
            projection,
        }
    }
    pub fn update_cam(&mut self, ratio: f32) -> &mut Self {
        self.projection = perspective(45.0, ratio, 0.1, 1e3);
        self.camera.update_motion();

        self
    }
    pub fn update_animations(&mut self, timer: &Timer) -> &mut Self {
        let center = vec3(0.0, 20.0, 20.0);
        let axis = vec3(0.0, 1.0, 0.0);
        let transform = &mut self.shapes.get_mut("cube2").unwrap().transform;

        basic::spin(timer.elapsed, 90.0, vec3(1.0, 1.0, 0.0), transform);

        basic::rotate_around(
            center,
            50.0,
            22.5,
            axis,
            timer.elapsed,
            &mut transform.translation,
        );

        self.player.update_animation(timer.elapsed);

        self
    }
    pub fn update_physics(&mut self) -> &mut Self {
        let shapes = &mut self.shapes;

        collision::sphere_sphere(String::from("ball"), String::from("ball2"), shapes);
        collision::sphere_aabb(String::from("ball"), String::from("platform"), shapes);
        collision::sphere_aabb(String::from("ball2"), String::from("platform"), shapes);
        collision::aabb_aabb(String::from("cube"), String::from("platform"), shapes);

        force::gravity(&mut shapes.get_mut("cube").unwrap().velocity);
        force::gravity(&mut shapes.get_mut("ball").unwrap().velocity);
        force::gravity(&mut shapes.get_mut("ball2").unwrap().velocity);

        self.shapes.values_mut().for_each(|shape| {
            shape.add_velocity();
        });

        self
    }

    pub fn update_shadows(&mut self) -> &mut Self {
        let shapes = &mut self.shapes;
        let shader = &mut self.shaders.get_mut("shadow").unwrap();

        self.sun.shadows.attach(1900, 1200);

        shader.set_use();
        shader.update_mat4("lightSpace", self.sun.transform());
        shader.update_mat4("model", self.player.transform.get());
        self.player.render();

        shapes.values_mut().for_each(|shape| {
            // shader.update_mat4("model", shape.transform.get());
            shape.render(shader);
        });
        // end of render
        shadows::Shadow::detach();

        self
    }

    pub fn render(&mut self) {
        let shapes = &mut self.shapes;
        let lights = &self.point_lights;
        let shader = &mut self.shaders.get_mut("object").unwrap();

        shader.set_use();
        self.sun.shadows.bind_texture();
        shader.update_vec3("L_direction", self.sun.dir);
        shader.update_vec3("L_color", self.sun.color);
        shader.update_vec3("viewPos", self.camera.pos);
        shader.update_mat4("view", self.camera.get_view());
        shader.update_mat4("projection", self.projection);
        shader.update_mat4("lightSpace", self.sun.transform());
        shader.update_int("shadowsEnabled", false as i32);

        let len = lights.len();

        shader.update_int("pointLightCount", len as i32);
        // update point lights
        for i in 0..len {
            pl_to_shader(lights[i], shader, i);
        }

        // object specific
        shapes.values_mut().for_each(|shape| {
            shape.render(shader);
        });
    }

    pub fn render_skeletal_animations(&mut self) {
        // let objects = &mut self.assets.objects;
        let lights = &self.point_lights;
        let shader = &mut self.shaders.get_mut("animation").unwrap();

        shader.set_use();
        self.sun.shadows.bind_texture();
        shader.update_vec3("L_direction", self.sun.dir);
        shader.update_vec3("L_color", self.sun.color);
        shader.update_vec3("viewPos", self.camera.pos);
        shader.update_mat4("view", self.camera.get_view());
        shader.update_mat4("projection", self.projection);
        shader.update_mat4("lightSpace", self.sun.transform());
        shader.update_int("shadowsEnabled", false as i32);

        let len = lights.len();

        shader.update_int("pointLightCount", len as i32);
        // update point lights
        for i in 0..len {
            pl_to_shader(lights[i], shader, i);
        }

        let mats = &self.player.get_pose();
        for i in 0..mats.len() {
            shader.update_mat4(format!("boneMats[{i}]").as_str(), mats[i]);
        }

        model_to_shader(&mut self.player, shader);
        self.player.render();
    }
}

// send player info to shader for drawing
fn model_to_shader(o: &mut Model, shader: &mut shaders::Program) {
    shader.update_mat4("transform", o.transform.get());
    shader.update_int("textured", o.textured as i32);
}

use shaders::{Program, Shader};

/// function assumes there will only be a vertex and fragment shader  
/// no geometry shader capabilities for this engine yet and not planning on adding anytime soon
fn create_shader(vert: &Path, frag: &Path) -> Program {
    Program::from_shaders(&[
        Shader::from_vert_src(&vert).unwrap(),
        Shader::from_frag_src(&frag).unwrap(),
    ])
    .unwrap()
}
/// send point light to shaders point light array
fn pl_to_shader(light: lights::PointLight, shader: &mut shaders::Program, i: usize) {
    let pos = format!("pointLights[{i}].position");
    let col = format!("pointLights[{i}].color");
    shader.update_vec3(pos.as_str(), light.pos);
    shader.update_vec3(col.as_str(), light.col);
}
