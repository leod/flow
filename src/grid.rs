use std::collections::{HashMap, hash_map};
use std::fmt::Debug;

pub use types::Coords;
use types::{ComponentId, Dir, PosDir};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point(pub ComponentId);

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
