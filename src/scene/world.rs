use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use super::camera::Camera;
use super::lights;
use super::lights::PointLight;
use crate::src::animation::*;
use crate::src::foreign::*;
use crate::src::renderer::model::Model;

use crate::src::renderer::shaders;
use crate::src::renderer::shadows;
use shaders::Program;

use crate::src::engine::timer::Timer;
use crate::src::math::{quaternion::Quat, vec3::*};

use crate::src::physics::collision;
use crate::src::physics::force;

use crate::src::shapes::{cube::cube, shape::Pattern, shape::Shape, sphere::*, torus::torus};

// abit messy but who cares
// not sure why im bothering with comments as if anyone is going to read any of this
pub struct World {
    pub camera: Camera,
    player: Model,
    pub sun: lights::DirectionalLight,
    pub shapes: HashMap<String, Shape>,    //done
    pub shaders: HashMap<String, Program>, //done
    pub point_lights: Vec<PointLight>,     //done
}

impl World {
    pub fn default() -> Self {
        let directional_light = lights::DirectionalLight {
            shadows: shadows::Shadow::new(800, 600),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };

        Self {
            camera: Camera::default(),
            player: Model::default(),
            sun: directional_light,
            shapes: HashMap::new(),
            shaders: HashMap::new(),
            point_lights: Vec::new(),
        }
    }

    pub fn new() -> Self {
        let mut camera = Camera::default();
        camera.pos = vec3(0.0, 20.0, -30.0);

        let mut shaders = HashMap::new();
        let mut shapes = HashMap::new();
        let mut point_lights = Vec::new();

        world_from_json(&mut shapes, &mut point_lights, &mut shaders);

        shapes.values_mut().for_each(|shape| {
            shape.create();
        });

        let sun = lights::DirectionalLight {
            shadows: shadows::Shadow::new(1900, 1200),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };

        let file = gltf::Gltf::new(Path::new("models/astronaut"));
        let mut player = Model::default();
        file.populate_model(&mut player);
        player
            .change_pos(vec3(0.0, 12.0, 3.0))
            .change_size(vec3(0.5, 0.5, 0.5));

        player.prepere_render_resources();
        player.transform.orientation = Quat::create(180.0, vec3(0.0, 1.0, 0.0));
        player.play_animation = true;
        player.current_anim = 0;

        Self {
            shapes,
            sun,
            camera,
            player,
            point_lights,
            shaders,
        }
    }

    pub fn add_shape(&mut self, name: String, shape: Shape) {
        if self.shapes.contains_key(&name) {
            println!("shape name already exists!");
        } else {
            self.shapes.insert(name, shape);
        }
    }

    pub fn add_shader(&mut self, name: String, shader: Program) {
        if self.shaders.contains_key(&name) {
            println!("shader name already exists!");
        } else {
            self.shaders.insert(name, shader);
        }
    }

    pub fn update_cam(&mut self) -> &mut Self {
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
        shader.update_mat4("lightSpace", &self.sun.transform());
        shader.update_mat4("model", &self.player.transform.to_mat());
        self.player.render(shader);

        shapes.values_mut().for_each(|shape| {
            // shader.update_mat4("model", shape.transform.get());
            shape.render(shader);
        });
        // end of render
        shadows::Shadow::detach();

        self
    }

    pub fn render(&mut self) {
        self.render_static_objects();
        self.render_skeletal_animations();
    }

    fn render_static_objects(&mut self) {
        let shapes = &mut self.shapes;
        let shader = &mut self.shaders.get_mut("object").unwrap();
        shader.set_use();
        // object specific
        shapes.values_mut().for_each(|shape| {
            shape.render(shader);
        });
    }

    fn render_skeletal_animations(&mut self) {
        let shader = &mut self.shaders.get_mut("animation").unwrap();
        shader.set_use();

        self.player.render(shader);
    }
}

extern crate json;
use std::fs::File;
fn world_from_json(
    shapes: &mut HashMap<String, Shape>,
    lights: &mut Vec<PointLight>,
    shaders: &mut HashMap<String, Program>,
) {
    //
    let mut file = File::open(Path::new("world.json")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = json::parse(&contents[..]).unwrap();

    use json::iterators::Members;
    //helper funtions to get the next values in the members in a json value
    fn get_f32(members: &mut Members) -> f32 {
        members.next().unwrap().as_f32().unwrap()
    }
    fn get_i32(members: &mut Members) -> i32 {
        members.next().unwrap().as_i32().unwrap()
    }

    let vec3_from_members = |members: &mut Members| -> Vec3 {
        vec3(get_f32(members), get_f32(members), get_f32(members))
    };

    //______________________________________________________________________
    //load shapes

    for shape in data["shapes"].members() {
        let shape_type = shape["type"].as_str().unwrap();
        let pattern_type = shape["pattern"]["type"].as_str().unwrap();

        let pos = vec3_from_members(&mut shape["position"].members());
        let scaling = vec3_from_members(&mut shape["scale"].members());
        let color = vec3_from_members(&mut shape["color"].members());

        let mut result = Shape::new();
        result.reposition(pos);
        result.rescale(scaling);

        match pattern_type {
            "checkered" => {
                let mut vals = shape["pattern"]["values"].members();
                let a = get_f32(&mut vals);
                let b = get_i32(&mut vals);
                result.change_pattern(Pattern::Checkered(a, b));
            }

            "striped" => {
                let mut values = shape["pattern"]["values"].members();
                let a = get_f32(&mut values);
                let b = get_f32(&mut values);
                let c = get_i32(&mut values);
                result.change_pattern(Pattern::Striped(a, b, c));
            }
            "none" => {
                //leave the pattern type to default value of none
            }

            _ => {
                println!("unknown pattern type {}", pattern_type);
            }
        }

        // yikes...
        // hey if it works it works
        match shape_type {
            "sphere" => {
                let lats = shape["lats"].as_u32().unwrap();
                let longs = shape["longs"].as_u32().unwrap();
                result.reshape(sphere(lats, longs, color));
            }

            "icosphere" => {
                let divs = shape["divs"].as_i32().unwrap();
                result.reshape(icosphere(divs, color));
            }

            "cube" => {
                let color_cube = shape["colorCube"].as_bool().unwrap();
                result.reshape(cube(color_cube, color));
            }

            "torus" => {
                let divs = shape["divs"].as_u32().unwrap();
                result.reshape(torus(divs, color));
            }

            _ => {
                println!("unknown shape type ({})!", shape_type);
            }
        }

        shapes.insert(shape["name"].to_string(), result);
    }

    //______________________________________________________________________
    //load point lights
    for light in data["lights"].members() {
        let pos = vec3_from_members(&mut light["pos"].members());
        let col = vec3_from_members(&mut light["col"].members());

        lights.push(PointLight { col, pos });
    }

    //______________________________________________________________________
    //load shaders
    for shader in data["shaders"].members() {
        let name = String::from(shader["name"].as_str().unwrap());
        let frag = Path::new(shader["frag"].as_str().unwrap());
        let vert = Path::new(shader["vert"].as_str().unwrap());

        shaders.insert(name, shaders::create_shader(&vert, &frag));
    }
}
