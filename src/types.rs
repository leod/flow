use std::cmp;

use cgmath;

pub type Coords = cgmath::Vector2<isize>;

pub type ComponentId = usize;

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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Rect {
    pub pos: Coords, // top left pos
    pub size: Coords
}

pub struct RectIter {
    rect: Rect,
    cur: Coords
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

    pub fn invert(self) -> Dir {
        self.rotate_cw().rotate_cw()
    }

    pub fn rotate_cw(self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left
        }
    }

    pub fn rotate_cw_n(self, n: usize) -> Dir {
        (0..n%4).fold(self, |d, _| d.rotate_cw())
    }

    pub fn apply(self, c: Coords) -> Coords {
        match self {
            Dir::Left => Coords::new(c.x - 1, c.y),
            Dir::Right => Coords::new(c.x + 1, c.y),
            Dir::Up => Coords::new(c.x, c.y - 1),
            Dir::Down => Coords::new(c.x, c.y + 1)
        }
    }

    pub fn apply_n(self, c: Coords, n: usize) -> Coords {
        (0..n).fold(c, |c, _| self.apply(c))
    }

    pub fn to_axis(self) -> Axis {
        match self {
            Dir::Left => Axis::Horizontal,
            Dir::Right => Axis::Horizontal,
            Dir::Up => Axis::Vertical,
            Dir::Down => Axis::Vertical
        }
    }

    pub fn is_pos(self) -> bool {
        match self {
            Dir::Left => false,
            Dir::Right => true,
            Dir::Up => false,
            Dir::Down => true
        }
    }
}

impl PosDir {
    pub fn to_dir(self) -> Dir {
        match self {
            PosDir::Right => Dir::Right,
            PosDir::Down => Dir::Down
        }
    }

    pub fn apply(self, c: Coords) -> Coords {
        match self {
            PosDir::Right => Coords::new(c.x + 1, c.y),
            PosDir::Down => Coords::new(c.x, c.y + 1)
        }
    }
}

impl Axis {
    pub fn invert(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal
        }
    }
}

impl Rect {
    pub fn from_coords(a: Coords, b: Coords) -> Rect {
        let pos = Coords::new(cmp::min(a.x, b.x),
                              cmp::min(a.y, b.y));
        let d = a.cast::<isize>() - b.cast();
        let size = Coords::new(d.x.abs(), d.y.abs());

        Rect {
            pos: pos,
            size: size
        }
    }

    // Iterate left-to-right, top-to-bottom
    pub fn iter(&self) -> RectIter {
        RectIter {
            rect: *self,
            cur: Coords::new(0, 0)
        }
    }

    pub fn rotate_cw(&self) -> Rect {
        Rect {
            pos: self.pos,
            size: Coords::new(self.size.y, self.size.x)
        }
    }

    pub fn rotate_cw_n(&self, n: usize) -> Rect {
        if n % 2 == 0 {
            *self
        }  else {
            self.rotate_cw()
        }
    }
}

impl Iterator for RectIter {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        if self.cur.y == self.rect.size.y || self.cur.x == self.rect.size.x {
            None
        } else {
            let p = self.rect.pos + self.cur;

            self.cur.x += 1;
            if self.cur.x == self.rect.size.x {
                self.cur.x = 0;
                self.cur.y += 1;
            }

            Some(p)
        }
    }
}
