#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(_x: f32, _y: f32) -> Vec2 {
    Vec2 { x: _x, y: _y }
}
