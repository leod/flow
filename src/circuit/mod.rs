mod action;
mod component;

use std::collections::HashMap;
use std::fmt::Debug;

use types::{Dir, PosDir};
use canon_map::{Canonize, CanonMap};

pub use types::Coords;
pub use self::action::Action;
pub use self::component::{Element, Component};

pub type ComponentId = usize;

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

impl Canonize for (Coords, Dir) {
    type Canon = (Coords, PosDir);

    fn canonize(&self) -> Self::Canon {
        match self.1 {
            Dir::Left => (self.1.apply(self.0), PosDir::Right),
            Dir::Right => (self.0, PosDir::Right),
            Dir::Up => (self.1.apply(self.0), PosDir::Down),
            Dir::Down => (self.0, PosDir::Down)
        }
    }
}

pub type EdgeMap = CanonMap<(Coords, Dir), Edge>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Circuit {
    points: HashMap<Coords, Point>,
    edges: EdgeMap,

    components: HashMap<ComponentId, Component>,
    next_component_id: ComponentId,
}

impl Circuit {
    pub fn empty() -> Circuit {
        Circuit {
            points: HashMap::new(),
            edges: EdgeMap::new(),
            components: HashMap::new(),
            next_component_id: 0,
        }
    }

    pub fn points(&self) -> &HashMap<Coords, Point> {
        &self.points
    }

    pub fn edges(&self) -> &EdgeMap {
        &self.edges
    }

    pub fn components(&self) -> &HashMap<ComponentId, Component> {
        &self.components
    }
}

pub struct EdgeDirIter<'a> {
    map: &'a EdgeMap,
    coords: Coords,
    cur: Option<Dir>
}

impl<'a> Iterator for EdgeDirIter<'a> {
    type Item = (Dir, Edge);

    fn next(&mut self) -> Option<(Dir, Edge)> {
        if let Some(dir) = self.cur {
            self.cur = match dir.rotate_cw() {
                           Dir::Up => None,
                           next_dir => Some(next_dir)
                       };

            self.map.get((self.coords, dir)).map(|edge| (dir, *edge))
        } else {
            None
        }
    }
}

impl EdgeMap {
    pub fn iter_dirs(&self, c: Coords) -> EdgeDirIter {
        EdgeDirIter {
            map: self,
            coords: c,
            cur: Some(Dir::Up)
        }
    }
}

