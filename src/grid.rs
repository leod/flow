use std::collections::{HashMap, hash_map};
use std::fmt::Debug;

pub use types::Coords;
use types::{ComponentId, Dir, PosDir};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Point {
    Node,
    Component(ComponentId)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Layer {
    Ground,
    Underground
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Edge {
    pub layer: Layer
}

#[derive(Clone, Debug)]
pub struct EdgeMap<T: Clone + Debug>(HashMap<(Coords, PosDir), T>);

fn canonize_edge(c: Coords, d: Dir) -> (Coords, PosDir) {
    match d {
        Dir::Left => (d.apply(c), PosDir::Right),
        Dir::Right => (c, PosDir::Right),
        Dir::Up => (d.apply(c), PosDir::Down),
        Dir::Down => (c, PosDir::Down)
    }
}

impl<T: Clone + Debug> EdgeMap<T> {
    pub fn new() -> EdgeMap<T> {
        EdgeMap(HashMap::new())
    }

    pub fn set(&mut self, c: Coords, d: Dir, t: T) {
        self.0.insert(canonize_edge(c, d), t);
    }
    
    pub fn get(&self, c: Coords, d: Dir) -> Option<&T> {
        self.0.get(&canonize_edge(c, d))
    }

    pub fn remove(&mut self, c: Coords, d: Dir)  {
        self.0.remove(&canonize_edge(c, d));
    }

    pub fn iter(&self) -> hash_map::Iter<(Coords, PosDir), T> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    points: HashMap<Coords, Point>,
    edges: EdgeMap<Edge>
}

impl Grid {
    pub fn new() -> Grid {
        return Grid {
            points: HashMap::new(),
            edges: EdgeMap::new()
        };
    }

    pub fn get_point(&self, c: Coords) -> Option<&Point> {
        self.points.get(&c)
    }

    pub fn set_point(&mut self, c: Coords, p: Point) {
        self.points.insert(c, p);
    }

    pub fn remove_point(&mut self, c: Coords) {
        self.points.remove(&c);
    }

    pub fn iter_points(&self) -> hash_map::Iter<Coords, Point> {
        self.points.iter()
    }

    pub fn get_edge(&self, c: Coords, d: Dir) -> Option<&Edge> {
        self.edges.get(c, d)
    }

    pub fn set_edge(&mut self, c: Coords, d: Dir, t: Edge) {
        self.edges.set(c, d, t);
    }

    pub fn remove_edge(&mut self, c: Coords, d: Dir) {
        self.edges.remove(c, d);
    }

    pub fn iter_edges(&self) -> hash_map::Iter<(Coords, PosDir), Edge> {
        return self.edges.iter()
    }
}
