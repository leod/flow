mod action;
mod component;

use std::collections::HashMap;
use std::fmt::Debug;

use types::{Dir, PosDir};
use canon_map::{Canonize, CanonMap};
use graph::NeighboredGraph;

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

type Point = (ComponentId, Option<usize>);

#[derive(Clone)]
pub struct Circuit {
    // Components of the circuit
    components: HashMap<ComponentId, Component>,

    // Cells, and edges between them.
    graph: NeighboredGraph<CellId, Coords, Edge>,

    // Grid coords that are occupied by components. Note that this can be
    // completely derived from the components. The point of this is to make it
    // easy for the hud to know which grid points are already in use.
    points: HashMap<Coords, Point>,

    // Counter to create unique component ids
    next_component_id: ComponentId,
}

impl Circuit {
    pub fn empty() -> Circuit {
        Circuit {
            components: HashMap::new(),
            graph: NeighboredGraph::new(),
            points: HashMap::new(),
            next_component_id: 0,
        }
    }

    pub fn points(&self) -> &HashMap<Coords, Point> {
        &self.points
    }

    pub fn graph(&self) -> &NeighboredGraph<CellId, Coords, Edge> {
        &self.graph
    }

    pub fn components(&self) -> &HashMap<ComponentId, Component> {
        &self.components
    }
}