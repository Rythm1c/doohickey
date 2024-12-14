extern crate gl;
use crate::src;
use crate::src::scene::camera::Direction;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn mouse_input(event: &Event, cam: &mut src::scene::camera::Camera) {
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
            Keycode::A => {
                cam.dir = Direction::Left;
            }

            Keycode::D => {
                cam.dir = Direction::Right;
            }

            Keycode::W => {
                cam.dir = Direction::Forwards;
            }

            Keycode::S => {
                cam.dir = Direction::Backwards;
            }

            _ => {}
        },
        Event::KeyUp {
            keycode: Some(key), ..
        } => match *key {
            Keycode::W | Keycode::S | Keycode::A | Keycode::D => {
                cam.dir = Direction::None;
            }

            _ => {}
        },

        _ => {}
    }
}
