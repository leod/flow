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
    Node,
    Switch(SwitchType),
    Source,
    Sink
}

pub struct ElementDescr {
    // Width and height. Each element occupies a rect of grid points.
    pub size: grid::Coords,

    // Potential input/output edges for this type of element, each described
    // by the side of the rect they are and the position on that side.
    // NOTE: edge_points is assumed not to contain duplicates. Also, the side 
    //       positions must be smaller than the size.
    pub edge_points: Vec<(Dir, usize)>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Component {
    pub top_left_pos: grid::Coords,
    pub element: Element,
    pub rotation: usize,
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        match self {
            &Element::Node => ElementDescr {
                size: grid::Coords::new(1, 1),
                edge_points: vec![(Dir::Left, 0), (Dir::Right, 0),
                                  (Dir::Up, 0), (Dir::Down, 0)],
            },
            &Element::Switch(_) => ElementDescr {
                size: grid::Coords::new(1, 1),
                edge_points: vec![(Dir::Left, 0), (Dir::Up, 0), (Dir::Down, 0)],
            },
            &Element::Source | &Element::Sink => ElementDescr {
                size: grid::Coords::new(3, 3),
                edge_points: vec![(Dir::Right, 1)],
            }
        }
    }
}
