extern crate gl;
use crate::src;
use crate::src::camera::Direction;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct WinInfo {
    pub running: bool,
    pub w: i32,
    pub h: i32,
}

impl WinInfo {
    /// get width to height ratio(w : h)
    pub fn get_ratio(&self) -> f32 {
        self.w as f32 / self.h as f32
    }
}

pub fn window_input(event: &Event, win_info: &mut WinInfo) {
    match event {
        Event::Quit { .. } => win_info.running = false,

        Event::Window {
            win_event: sdl2::event::WindowEvent::Resized(w, h),
            ..
        } => {
            win_info.w = *w;
            win_info.h = *h;
        }
        _ => {}
    }
}
pub fn mouse_input(event: &Event, cam: &mut src::camera::Camera) {
    match event {
        Event::MouseMotion {
            xrel,
            yrel,
            mousestate,
            ..
        } => {
            if mousestate.left() {
                cam.rotate(*xrel, -*yrel);
            }
        }

        Event::KeyDown {
            keycode: Some(key), ..
        } => match *key {
            Keycode::A => cam.dir = Direction::Left,
            Keycode::D => cam.dir = Direction::Right,
            Keycode::W => cam.dir = Direction::Forwards,
            Keycode::S => cam.dir = Direction::Backwards,

            _ => {}
        },
        Event::KeyUp {
            keycode: Some(key), ..
        } => match *key {
            Keycode::W | Keycode::S | Keycode::A | Keycode::D => cam.dir = Direction::None,

            _ => {}
        },

        _ => {}
    }
}
