use std::iter::*;
use std::slice;

pub use types::Coords;
use types::{ComponentId, Orientation};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Point {
    Empty,
    Component(ComponentId)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Layer {
    Ground,
    Underground
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Edge {
    Empty,
    Connected(Layer)
}

#[derive(Clone, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    points: Vec<Point>,
    right_edges: Vec<Edge>,
    down_edges: Vec<Edge>
}

pub struct RectIter {
    origin: Coords,
    width: usize,
    height: usize,

    c: Coords
}

impl RectIter {
    pub fn new(origin: Coords, width: usize, height: usize) -> RectIter {
        RectIter {
            origin: origin,
            width: width,
            height: height,
            c: origin
        }
    }
}

impl Iterator for RectIter {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        if self.c.x == self.origin.x && self.c.y == self.origin.y + self.height {
            None
        } else {
            let r = self.c;

            self.c.x += 1;
            if self.c.x == self.origin.x + self.width {
                self.c.x = self.origin.x;
                self.c.y += 1;
            }

            Some(r)
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        assert!(width > 0 && height > 0, "can't create empty grid");

        return Grid {
            width: width,
            height: height,
            points: vec![Point::Empty; width * height],
            right_edges: vec![Edge::Empty; (width-1) * height],
            down_edges: vec![Edge::Empty; width * (height-1)]
        };
    }

    pub fn width(self: &Grid) -> usize {
        self.width
    }

    pub fn height(self: &Grid) -> usize {
        self.height
    }

    pub fn is_coord(self: &Grid, c: Coords) -> bool {
        c.x < self.width && c.y < self.height
    }

    pub fn point(self: &Grid, c: Coords) -> Point {
        assert!(c.x < self.width);
        assert!(c.y < self.height);

        self.points[c.y * self.width + c.x]
    }

    pub fn set_point(self: &mut Grid, c: Coords, p: Point) {
        assert!(c.x < self.width);
        assert!(c.y < self.height);

        self.points[c.y * self.width + c.x] = p;
    }

    pub fn right_edge(self: &Grid, c: Coords) -> Edge {
        assert!(c.x + 1 < self.width);
        assert!(c.y < self.height);

        self.right_edges[c.y * (self.width-1) + c.x]
    }
    
    pub fn down_edge(self: &Grid, c: Coords) -> Edge {
        assert!(c.x < self.width);
        assert!(c.y + 1 < self.height);

        self.down_edges[c.y * self.width + c.x]
    }

    pub fn right_edge_mut(self: &mut Grid, c: Coords) -> &mut Edge {
        assert!(c.x + 1 < self.width);
        assert!(c.y < self.height);

        &mut self.right_edges[c.y * (self.width-1) + c.x]
    }
    
    pub fn down_edge_mut(self: &mut Grid, c: Coords) -> &mut Edge {
        assert!(c.x < self.width);
        assert!(c.y + 1 < self.height);

        &mut self.down_edges[c.y * self.width + c.x]
    }

    pub fn edge(self: &Grid, c: Coords, o: Orientation) -> Edge {
        match o {
            Orientation::Left => self.right_edge(o.apply(c)),
            Orientation::Right => self.right_edge(c),
            Orientation::Up => self.down_edge(o.apply(c)),
            Orientation::Down => self.down_edge(c)
        }
    }

    fn edge_mut(self: &mut Grid, c: Coords, o: Orientation) -> &mut Edge {
        match o {
            Orientation::Left => self.right_edge_mut(o.apply(c)),
            Orientation::Right => self.right_edge_mut(c),
            Orientation::Up => self.down_edge_mut(o.apply(c)),
            Orientation::Down => self.down_edge_mut(c)
        }
    }

    pub fn set_edge(self: &mut Grid, c: Coords, o: Orientation, e: Edge) {
        *self.edge_mut(c, o) = e;
    }

    pub fn edges_iter(self: &Grid)
        -> Chain<Zip<RectIter, Zip<Cycle<Once<Orientation>>, slice::Iter<Edge>>>,
                 Zip<RectIter, Zip<Cycle<Once<Orientation>>, slice::Iter<Edge>>>> {
        let right = RectIter::new(Coords::new(0, 0), self.width-1, self.height)
            .zip(once(Orientation::Right).cycle()
                 .zip(self.right_edges.iter()));
        let down = RectIter::new(Coords::new(0, 0), self.width, self.height-1)
            .zip(once(Orientation::Down).cycle()
                 .zip(self.down_edges.iter()));

        down.chain(right)
    }
}
