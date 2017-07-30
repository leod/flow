use std::cmp;
use std::slice;

use cgmath;

pub type Coords = cgmath::Vector2<isize>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Rect {
    pub pos: Coords, // top left pos
    pub size: Coords,
}

pub struct RectIter {
    rect: Rect,
    cur: Coords,
}

#[allow(dead_code)]
impl Dir {
    pub fn iter() -> slice::Iter<'static, Dir> {
        static DIRS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
        DIRS.into_iter()
    }

    // Direction pointing from a to b
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
            panic!("a == b or even worse");
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
            Dir::Down => Dir::Left,
        }
    }

    pub fn rotate_cw_n(self, n: usize) -> Dir {
        (0..n % 4).fold(self, |d, _| d.rotate_cw())
    }

    pub fn delta(self) -> Coords {
        match self {
            Dir::Left => Coords::new(-1, 0),
            Dir::Right => Coords::new(1, 0),
            Dir::Up => Coords::new(0, -1),
            Dir::Down => Coords::new(0, 1),
        }
    }

    pub fn apply(self, c: Coords) -> Coords {
        c + self.delta()
    }

    pub fn apply_n(self, c: Coords, n: usize) -> Coords {
        (0..n).fold(c, |c, _| self.apply(c))
    }

    pub fn to_axis(self) -> Axis {
        match self {
            Dir::Left => Axis::Horizontal,
            Dir::Right => Axis::Horizontal,
            Dir::Up => Axis::Vertical,
            Dir::Down => Axis::Vertical,
        }
    }
}

impl Axis {
    pub fn invert(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[allow(dead_code)]
impl Rect {
    pub fn from_coords(a: Coords, b: Coords) -> Rect {
        let pos = Coords::new(cmp::min(a.x, b.x), cmp::min(a.y, b.y));
        let d = a.cast::<isize>() - b.cast();
        let size = Coords::new(d.x.abs(), d.y.abs());

        Rect {
            pos: pos,
            size: size,
        }
    }

    // Iterate left-to-right, top-to-bottom
    pub fn iter(&self) -> RectIter {
        RectIter {
            rect: *self,
            cur: Coords::new(0, 0),
        }
    }

    pub fn rotate(&self) -> Rect {
        Rect {
            pos: self.pos,
            size: Coords::new(self.size.y, self.size.x),
        }
    }

    pub fn rotate_n(&self, n: usize) -> Rect {
        if n % 2 == 0 { *self } else { self.rotate() }
    }

    pub fn first_corner_cw(&self, dir: Dir) -> Coords {
        match dir {
            Dir::Up => self.pos + Coords::new(0, 0),
            Dir::Right => self.pos + Coords::new(self.size.x, 0),
            Dir::Down => self.pos + Coords::new(self.size.x, self.size.y),
            Dir::Left => self.pos + Coords::new(0, self.size.y),
        }
    }

    pub fn is_within(&self, c: Coords) -> bool {
        return c.x >= self.pos.x && c.x <= self.pos.x + self.size.x && c.y >= self.pos.y &&
            c.y <= self.pos.y + self.size.y;
    }
}

impl Iterator for RectIter {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        if self.cur.y == self.rect.size.y + 1 || self.cur.x == self.rect.size.x + 1 {
            None
        } else {
            let p = self.rect.pos + self.cur;

            self.cur.x += 1;
            if self.cur.x == self.rect.size.x + 1 {
                self.cur.x = 0;
                self.cur.y += 1;
            }

            Some(p)
        }
    }
}
