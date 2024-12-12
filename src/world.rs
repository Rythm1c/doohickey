use std::collections::HashMap;
use std::path::Path;

use super::animation::*;
use super::camera::Camera;
use super::foreign::*;
use super::lights;
use super::lights::PointLight;
use super::model::Model;
use shaders::Program;

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

        let s_obj = shaders::create_shader(
            Path::new("shaders/common.vert"),
            Path::new("shaders/phong.frag"),
        );

        let s_shadow = shaders::create_shader(
            Path::new("shaders/shadowmap.vert"),
            Path::new("shaders/shadowmap.frag"),
        );

        let s_animation = shaders::create_shader(
            Path::new("shaders/animation.vert"),
            Path::new("shaders/phong.frag"),
        );

        let mut shaders = HashMap::new();
        shaders.insert(String::from("object"), s_obj);
        shaders.insert(String::from("shadow"), s_shadow);
        shaders.insert(String::from("animation"), s_animation);

        let mut shapes = HashMap::new();
        let mut point_lights = Vec::new();

        world_from_json(&mut shapes, &mut point_lights);
        shapes
            .get_mut("ball")
            .unwrap()
            .change_pattern(Pattern::Checkered(0.1, 30));

        shapes
            .get_mut("platform")
            .unwrap()
            .change_pattern(Pattern::Striped(0.3, 0.005, 70));

        shapes.values_mut().for_each(|shape| {
            shape.create();
        });

        let sun = lights::DirectionalLight {
            shadows: shadows::Shadow::new(1900, 1200),
            color: vec3(1.0, 1.0, 1.0),
            dir: vec3(0.3, -0.7, 0.4),
        };

        let mut player = Model::default();
        let file = gltf::GltfFile::new(Path::new("models/penguin/scene.gltf"));
        file.extract_textures();
        player.update_albedo(Path::new("models/astronaut/textures/m_main_baseColor.png"));

        player.meshes = file.extract_meshes();
        player.skeleton.rest_pose = file.extract_rest_pose();
        player.skeleton.inverse_bind_pose = file.extract_inverse_bind_mats();
        player.skeleton.joint_names = file.extract_joint_names();
        player.animations = file.extract_animations();
        player
            .change_pos(vec3(0.0, 12.0, 3.0))
            .change_size(vec3(7.5, 7.5, 7.5));

        player.prepere_render_resources();
        player.transform.orientation = Quat::create(180.0, vec3(0.0, 1.0, 0.0));
        player.play_animation = true;
        player.current_anim = 0;
        player.textured = false;

        let projection = perspective(45.0, ratio, 0.1, 1e3);

        // prepare the textures in the shaders for rendering
        {
            let obj_shader = shaders.get_mut("object").unwrap();

            obj_shader.set_use();
            obj_shader.update_int("shadowMap", 0);
            obj_shader.update_int("albedo", 1);
            obj_shader.update_int("specular", 2);
        }

        {
            let anim_shader = shaders.get_mut("animation").unwrap();

            anim_shader.set_use();
            anim_shader.update_int("shadowMap", 0);
            anim_shader.update_int("albedo", 1);
            anim_shader.update_int("specular", 2);
        }

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

        // send player info to shader for drawing
        shader.update_mat4("transform", self.player.transform.get());
        shader.update_int("textured", self.player.textured as i32);
        self.player.attach_albedo();

        self.player.render();
    }
}

/// send point light to shaders point light array
fn pl_to_shader(light: lights::PointLight, shader: &mut shaders::Program, i: usize) {
    let pos = format!("pointLights[{i}].position");
    let col = format!("pointLights[{i}].color");
    shader.update_vec3(pos.as_str(), light.pos);
    shader.update_vec3(col.as_str(), light.col);
}

extern crate json;

pub fn world_from_json(shapes: &mut HashMap<String, Shape>, lights: &mut Vec<PointLight>) {
    let data = json::object! {
        "count": 6,
        "shapes": [
            {
                "type": "sphere",
                "lats": 200,
                "longs": 200,
                "name": "ball",
                "scale": [ 4.0,4.0,4.0 ],
                "position": [ 4.0, 30.0,10.0],
                "color": [ 1.0,1.0,1.0 ],
                "pattern":
                {
                    "checkered": [ 0.3, 20]
                }
            },

            {
                "type": "icosphere",
                "divs": 4,
                "name": "ball2",
                "scale": [ 7.0,7.0,7.0],
                "position": [15.0, 40.0, 10.0],
                "color": [ 1.0, 0.35, 0.06],
                "pattern": "none"
            },

            {
                "type": "cube",
                "colorCube": false,
                "name": "cube",
                "scale": [ 6.0,6.0,6.0 ],
                "position": [-15.0,40.0,20.0],
                "color": [1.0, 0.13, 0.48],
                "pattern": "none"
            },
            {
                "type": "cube",
                "colorCube": true,
                "name": "cube2",
                "scale": [  5.0, 5.0, 5.0 ],
                "position": [ 5.0, 5.0, 5.0 ],
                "color": [0.0, 0.0, 0.0],
                "pattern": "none"
            },

            {
                "type": "torus",
                "divs": 60,
                "name": "torus",
                "scale": [10.0,10.0,10.0],
                "position": [ -15.0, 5.0,-5.0 ],
                "color": [ 0.64, 1.0,0.13 ],
                "pattern": "none"
            },

            {
                "type": "cube",
                "colorCube": false,
                "name": "platform",
                "scale": [ 1000.0, 2.0, 1000.0 ],
                "position": [0.0,-2.0, 0.0 ],
                "color": [ 0.9, 0.9, 0.9],
                "pattern":
                {
                    "striped": [0.1, 0.005,70]
                }
            }
        ],

        "lights": [
            {
                "pos": [30.0, 20.0, -20.0],
                "col": [1.0, 1.0, 1.0]
            },
            {
                "pos": [-30.0, 20.0, -20.0],
                "col": [1.0, 0.6, 0.01]
            },
            {
                "pos": [30.0, 20.0, 40.0],
                "col": [1.0, 0.0, 1.0]
            },
            {
                "pos": [-30.0, 20.0, 40.0],
                "col": [0.0, 1.0, 0.5]
            }

        ],

        "shaders": [
            {
                "name" : "object",
                "frag" : "shaders/phong.frag",
                "vert" : "shaders/common.vert"
            },
            {
                "name" : "shadowMap",
                "frag" : "shaders/shadowmap.frag",
                "vert" : "shaders/shadowmap.vert"
            },
            {
                "name" : "animation",
                "frag" : "shaders/phong.frag",
                "vert" : "shaders/animation.vert"
            },
        ]

    };

    let count = data["count"].as_usize().unwrap();

    for i in 0..count {
        let shape = &data["shapes"][i];
        let _type = shape["type"].as_str().unwrap();

        let mut result = Shape::new();

        let mut pos = shape["position"].members();
        result.reposition(vec3(
            pos.next().unwrap().as_f32().unwrap(),
            pos.next().unwrap().as_f32().unwrap(),
            pos.next().unwrap().as_f32().unwrap(),
        ));

        let mut scaling = shape["scale"].members();
        result.rescale(vec3(
            scaling.next().unwrap().as_f32().unwrap(),
            scaling.next().unwrap().as_f32().unwrap(),
            scaling.next().unwrap().as_f32().unwrap(),
        ));

        let mut color = shape["color"].members();

        // yikes...
        // hey if it works it works
        match _type {
            "sphere" => {
                let lats = shape["lats"].as_u32().unwrap();
                let longs = shape["longs"].as_u32().unwrap();
                let col = vec3(
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                );
                result.reshape(sphere(lats, longs, col));
            }

            "icosphere" => {
                let divs = shape["divs"].as_i32().unwrap();
                let col = vec3(
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                );
                result.reshape(icosphere(divs, col));
            }

            "cube" => {
                let color_cube = shape["colorCube"].as_bool().unwrap();
                let col = vec3(
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                );
                result.reshape(cube(color_cube, col));
            }

            "torus" => {
                let divs = shape["divs"].as_u32().unwrap();
                let col = vec3(
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                    color.next().unwrap().as_f32().unwrap(),
                );
                result.reshape(torus(divs, col));
            }

            _ => {
                println!("unknown shape type ({})!", _type);
            }
        }

        shapes.insert(shape["name"].to_string(), result);
    }

    for light in data["lights"].members() {
        let mut position = light["pos"].members();
        let mut color = light["col"].members();

        lights.push(PointLight {
            col: vec3(
                color.next().unwrap().as_f32().unwrap(),
                color.next().unwrap().as_f32().unwrap(),
                color.next().unwrap().as_f32().unwrap(),
            ),
            pos: vec3(
                position.next().unwrap().as_f32().unwrap(),
                position.next().unwrap().as_f32().unwrap(),
                position.next().unwrap().as_f32().unwrap(),
            ),
        });
    }
}
