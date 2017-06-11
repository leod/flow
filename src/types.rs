use cgmath;

pub type Coords = cgmath::Vector2<usize>;
pub type ComponentId = u32;

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

use self::Orientation::*;

impl Orientation {
    pub fn invert(self: Orientation) -> Orientation {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up
        }
    }

    pub fn apply(self: Orientation, c: Coords) -> Coords {
        match self {
            Left => Coords::new(c.x - 1, c.y),
            Right => Coords::new(c.x + 1, c.y),
            Up => Coords::new(c.x, c.y - 1),
            Down => Coords::new(c.x, c.y + 1)
        }
    }
}
