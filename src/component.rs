use std::collections::HashSet;

use component;

pub use types::ComponentId;
use types::Dir;
use grid;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SwitchType {
    On,
    Off
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Element {
    Switch(SwitchType),
    Source,
    Sink
}

pub struct ElementDescr {
    // Width and height. Each element occupies a rect of grid points.
    size: grid::Coords,

    // Potential input/output edges for this type of element, each described
    // by the side of the rect they are and the position on that side.
    // NOTE: edge_points is assumed not to contain duplicates. Also, the side 
    //       positions must be smaller than the size.
    edge_points: Vec<(Dir, usize)>
}

pub type Id = u32;

#[derive(Clone, Debug)]
pub struct Component {
    id: Id,
    top_left_position: grid::Coords,
    rotation: usize,
    element: Element
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        match self {
            &Element::Switch(_) => ElementDescr {
                size: grid::Coords::new(1, 1),
                edge_points: vec![(Dir::Left, 0), (Dir::Up, 0), (Dir::Down, 0)]
            },
            &Element::Source | &Element::Sink => ElementDescr {
                size: grid::Coords::new(3, 3),
                edge_points: vec![(Dir::Right, 1)]
            }
        }
    }
}
