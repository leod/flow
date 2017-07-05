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

// Each component consists of cells where edges can attach. The cells are
// created at the edge points of the component, which are described by its
// element.
pub type CellId = (ComponentId, usize);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Layer {
    Ground,
    Underground
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Edge {
    pub layer: Layer
}

pub type EdgeMap = CanonMap<(CellId, CellId), Edge>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Circuit {
    // Components of the circuit
    components: HashMap<ComponentId, Component>,

    // Edges between cells of two different components
    edges: EdgeMap,

    // Grid coords that are occupied by components. Note that this can be
    // completely derived from the components. The point of this is to make it
    // easy for the hud to know which grid points are already in use.
    points: HashMap<Coords, ComponentId>,

    // Counter to create unique component ids
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

    pub fn points(&self) -> &HashMap<Coords, ComponentId> {
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

