use crate::math::{mat4::*, misc::*, vec3::*};

pub enum Direction {
    None,
    Forwards,
    Backwards,
    Left,
    Right,
}
pub struct Camera {
    yaw: f32,
    pitch: f32,

    pub front: Vec3,
    pub up: Vec3,
    pub pos: Vec3,
    pub velocity: f32,
    pub dir: Direction,
}

impl Camera {
    pub fn new(f: Vec3, u: Vec3, p: Vec3, v: f32) -> Result<Camera, String> {
        Ok(Camera {
            front: f,
            up: u,
            pos: p,
            velocity: v,
            pitch: 0.0,
            yaw: radians(90.0),
            dir: Direction::None,
        })
    }

    pub fn get_view_mat(&self) -> Mat4 {
        look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn rotate(&mut self, mouse_pos_x: i32, mouse_pos_y: i32) {
        let xoffset: f32 = 0.15 * (mouse_pos_x) as f32;
        let yoffset: f32 = 0.15 * (mouse_pos_y) as f32;

        self.yaw += radians(xoffset);
        self.pitch += radians(yoffset);

        clamp(self.pitch, radians(-89.0), radians(89.0));

        let mut new_front = vec3(0.0, 0.0, 0.0);

        new_front.x = self.pitch.cos() * self.yaw.cos();
        new_front.y = self.pitch.sin();
        new_front.z = self.pitch.cos() * self.yaw.sin();

        self.front = new_front.unit();
    }

    fn back(&mut self) {
        self.pos = self.pos - self.front * self.velocity;
    }
    fn forward(&mut self) {
        self.pos = self.pos + self.front * self.velocity;
    }
    fn left(&mut self) {
        self.pos = self.pos + cross(&self.up, &self.front).unit() * self.velocity;
    }
    fn right(&mut self) {
        self.pos = self.pos - cross(&self.up, &self.front).unit() * self.velocity;
    }

    pub fn update_motion(&mut self) {
        match self.dir {
            //don't move
            Direction::None => {}

            Direction::Left => self.left(),

            Direction::Right => self.right(),

            Direction::Backwards => self.back(),

            Direction::Forwards => self.forward(),
            // _ => {}
        }
    }
}
