use cgmath;

pub type Coords = cgmath::Vector2<usize>;

pub type ComponentId = u32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

// Only directions that increase a coordinate
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PosDir {
    PosRight,
    PosDown
}

impl Dir {
    pub fn invert(self: Dir) -> Dir {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up
        }
    }

    pub fn apply(self: Dir, c: Coords) -> Coords {
        match self {
            Dir::Left => Coords::new(c.x - 1, c.y),
            Dir::Right => Coords::new(c.x + 1, c.y),
            Dir::Up => Coords::new(c.x, c.y - 1),
            Dir::Down => Coords::new(c.x, c.y + 1)
        }
    }
}

impl PosDir {
    pub fn to_dir(self: PosDir) -> Dir {
        match self {
            PosDir::PosRight => Dir::Right,
            PosDir::PosDown => Dir::Down
        }
    }
}
