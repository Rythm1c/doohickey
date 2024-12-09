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
    pub sensitivity: f32,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            front: vec3(0.0, 0.0, 1.0),
            up: vec3(0.0, 1.0, 0.0),
            pos: vec3(0.0, 4.0, 0.0),
            velocity: 0.5,
            pitch: 0.0,
            yaw: radians(90.0),
            sensitivity: 0.15,
            dir: Direction::None,
        }
    }

    pub fn new(f: Vec3, u: Vec3, p: Vec3, v: f32) -> Self {
        Camera {
            front: f,
            up: u,
            pos: p,
            velocity: v,
            ..Camera::default()
        }
    }

    pub fn get_view(&self) -> Mat4 {
        look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn rotate(&mut self, mouse_pos_x: i32, mouse_pos_y: i32) {
        let xoffset = self.sensitivity * (mouse_pos_x) as f32;
        let yoffset = self.sensitivity * (mouse_pos_y) as f32;

        self.yaw += radians(xoffset);
        self.pitch += radians(yoffset);

        clamp(self.pitch, radians(-89.0), radians(89.0));

        let mut new_front = vec3(0.0, 0.0, 0.0);

        new_front.x = f32::cos(self.pitch) * f32::cos(self.yaw);
        new_front.y = f32::sin(self.pitch);
        new_front.z = f32::cos(self.pitch) * f32::sin(self.yaw);

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
