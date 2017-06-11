pub use types::Coords;
use types::{ComponentId, Orientation};

#[derive(Clone, Copy, Debug)]
pub enum Point {
    Empty,
    Component(ComponentId)
}

#[derive(Clone, Copy, Debug)]
pub enum Layer {
    Ground,
    Underground
}

#[derive(Clone, Copy, Debug)]
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
            Left => self.right_edge(o.apply(c)),
            Right => self.right_edge(c),
            Up => self.down_edge(o.apply(c)),
            Down => self.down_edge(c)
        }
    }

    pub fn edge_mut(self: &mut Grid, c: Coords, o: Orientation) -> &mut Edge {
        match o {
            Left => self.right_edge_mut(o.apply(c)),
            Right => self.right_edge_mut(c),
            Up => self.down_edge_mut(o.apply(c)),
            Down => self.down_edge_mut(c)
        }
    }

    pub fn set_edge(self: &mut Grid, c: Coords, o: Orientation, e: Edge) {
        *self.edge_mut(c, o) = e;
    }
}
