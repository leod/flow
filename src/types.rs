use cgmath;

pub type Coords = cgmath::Vector2<isize>;

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
    Right,
    Down
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Axis {
    Horizontal,
    Vertical
}

impl Dir {
    pub fn from_coords(a: Coords, b: Coords) -> Dir {
        if b.x == a.x - 1 {
            assert!(a.y == b.y);
            Dir::Left
        } else if b.x == a.x + 1 {
            assert!(a.y == b.y);
            Dir::Right
        } else if b.y == a.y - 1 {
            assert!(a.x == b.x);
            Dir::Up
        } else if b.y == a.y + 1 {
            assert!(a.x == b.x);
            Dir::Down
        } else {
            panic!("a == b");
        }
    }

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

    pub fn to_axis(self: Dir) -> Axis {
        match self {
            Dir::Left => Axis::Horizontal,
            Dir::Right => Axis::Horizontal,
            Dir::Up => Axis::Vertical,
            Dir::Down => Axis::Vertical
        }
    }
}

impl PosDir {
    pub fn to_dir(self: PosDir) -> Dir {
        match self {
            PosDir::Right => Dir::Right,
            PosDir::Down => Dir::Down
        }
    }

    pub fn apply(self: PosDir, c: Coords) -> Coords {
        match self {
            PosDir::Right => Coords::new(c.x + 1, c.y),
            PosDir::Down => Coords::new(c.x, c.y + 1)
        }
    }
}
