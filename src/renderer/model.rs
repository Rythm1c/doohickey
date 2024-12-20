use std::path::Path;

use crate::src::math::mat4::Mat4;
use crate::src::math::transform::Transform;
use crate::src::math::vec3::Vec3;

use super::material::*;
use super::mesh::*;
use super::shaders;

use crate::src::animation::clip::Clip;
use crate::src::animation::pose::Pose;
use crate::src::animation::skeleton::Skeleton;

use super::texture::Texture;

// i seriously need to refactor this mess
#[derive(Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub transform: Transform,
    pub velocity: Vec3,
    pub animations: Vec<Clip>, //optional
    pub skeleton: Skeleton,    //optional
    pub current_anim: usize,   //refactor
    pub play_animation: bool,  //refactor
    pub final_pose: Pose,      //refactor
    pub material: Material,
    pub textures: Vec<Texture>,
    pub textured: bool,
    albedo: Texture, //refactor
}

impl Model {
    pub fn default() -> Self {
        Self {
            transform: Transform::DEFAULT,
            meshes: Vec::new(),
            textured: false,
            velocity: Vec3::ZERO,
            skeleton: Skeleton::new(),
            animations: Vec::new(),
            play_animation: false,
            current_anim: 0,
            final_pose: Pose::new(),
            albedo: Texture::new(),
            textures: Vec::new(),
            material: Material::BlinnPhong(Phong::default()),
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn prepere_render_resources(&mut self) {
        for mesh in self.meshes.iter_mut() {
            mesh.create();
        }
    }

    pub fn update_albedo(&mut self, path: &Path) {
        self.albedo.from(path);
    }

    pub fn change_pos(&mut self, n_pos: Vec3) -> &mut Self {
        self.transform.translation = n_pos;
        self
    }

    pub fn change_size(&mut self, n_size: Vec3) -> &mut Self {
        self.transform.scaling = n_size;
        self
    }

    pub fn add_velocity(&mut self) {
        self.transform.translation = self.transform.translation + self.velocity;
    }

    // can only choose one lighting model per model
    // for now
    pub fn render(&mut self, shader: &mut shaders::Program) {
        // use correct material

        // to be continued...
        match &self.material {
            Material::Pbr(..) => {
                //something
            }

            Material::BlinnPhong(..) => {
                //something
            }
        }
        shader.update_mat4("transform", self.transform.to_mat());
        shader.update_int("textured", self.textured as i32);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
        }
        self.albedo.bind();

        for mesh in self.meshes.iter_mut() {
            mesh.render();
        }
    }
    pub fn recolor(&mut self, color: Vec3) {
        self.meshes.iter_mut().for_each(|mesh| {
            mesh.vao.vertices.iter_mut().for_each(|vertex| {
                vertex.col = color;
            });
        });
    }

    pub fn update_animation(&mut self, time: f32) {
        if self.play_animation {
            self.final_pose = self.skeleton.rest_pose.clone();
            // extract animation for each joint(bone)
            self.animations[self.current_anim].sample(&mut self.final_pose, time);
        }
    }

    pub fn get_pose(&mut self) -> Vec<Mat4> {
        let mut final_mats = Vec::new();

        let len = self.skeleton.rest_pose.joints.len();
        //black holders for mats which arent being used
        final_mats.resize(len, Mat4::IDENTITY);

        if self.play_animation {
            // the match statements are a slight attempt at optimization, to reduce the amount of matrix multiplication going on ofcourse
            // only multiply with the joint that are actually sent to the shader
            // not sure if this is significant though
            for i in 0..len {
                if let Some(inverse_pose) = self.skeleton.inverse_bind_pose[i] {
                    // only get global transforms for joints being used
                    // to reduce iterations perfomed ofcourse
                    let world = self.final_pose.get_global_tranform(i);

                    final_mats[i] = world.to_mat() * inverse_pose;
                } else {
                    // do nothing if joint contains no inverse bind pose
                    // final_mats[i] = world.to_mat() ;
                }
            }
        } else {
            for i in 0..len {
                // only get global transforms for joints being used
                if let Some(inverse_pose) = self.skeleton.inverse_bind_pose[i] {
                    let world = self.skeleton.rest_pose.get_global_tranform(i);

                    final_mats[i] = world.to_mat() * inverse_pose;
                } else {
                    // do nothing if joint contains no inverse bind pose
                    // final_mats[i] = world.to_mat();
                }
            }
        }

        final_mats
    }
}
