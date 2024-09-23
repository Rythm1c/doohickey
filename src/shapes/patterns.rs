use crate::math::vec2::*;

pub fn shaded(texel: Vec2, pattern: Pattern) -> f32 {
    match pattern {
        Pattern::Checkered(darkness, squares) => {
            let square = 2.0 / squares as f32;

            let edge = vec2(0.5, 0.5);

            let x = f32::fract(texel.x / square);
            let y = f32::fract(texel.y / square);
            //location
            let texel = vec2(x, y);
            let value = Vec2::step(&edge, &texel);

            if ((value[0] + value[1]) % 2) == 1 {
                return darkness;
            } else {
                1.0
            }
        }

        Pattern::Striped(darkness, thickness, lines) => {
            let line = 1.0 / lines as f32;

            let edge = vec2(thickness, thickness);

            let x = f32::fract(texel.x / line);
            let y = f32::fract(texel.y / line);

            let a = Vec2::step(&edge, &vec2(x, y));
            let b = Vec2::step(&edge, &vec2(1.0 - x, 1.0 - y));

            if (a[0] * a[1] * b[0] * b[1]) == 0 {
                return darkness;
            } else {
                1.0
            }
        }
    }
}

/// choose what pattern to give to a shape
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    /// drakness, number of squares
    Checkered(f32, i32),

    /// darkness, thickness, number of lines
    Striped(f32, f32, i32),
}
