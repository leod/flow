use std::collections::HashSet;

use types::{Dir, Rect};
use circuit;

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
    pub size: circuit::Coords,

    pub edge_points: Vec<(Dir, Vec<usize>)>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Component {
    pub element: Element,

    // Position of the top left corner in the circuit this component belongs to
    pub pos: circuit::Coords,

    // Number of clockwise rotations
    pub rotation_cw: usize,

    // Derived quantities:
    pub rect: Rect,

    // Same order as in the element description
    pub edges: Vec<(circuit::Coords, Dir)>,

    // Unique positions of edges
    pub edge_points: Vec<circuit::Coords>
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        match *self {
            Element::Node => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edges: vec![(Dir::Left, 0), (Dir::Right, 0),
                                  (Dir::Up, 0), (Dir::Down, 0)],
            },
            Element::Switch(_) => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edges: vec![(Dir::Left, 0), (Dir::Up, 0), (Dir::Down, 0)],
            },
            Element::Source => ElementDescr {
                size: circuit::Coords::new(2, 2),
                edges: vec![(Dir::Right, 1)],
            },
            Element::Sink => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edges: vec![(Dir::Right, 0)]
            },
        }
    }

    pub fn new_component(
        &self,
        top_left_pos: circuit::Coords,
        rotation_cw: usize
    ) -> Component {
        let descr = self.descr();
        let size = descr.size;
        let rect =
            Rect {
                pos: top_left_pos,
                size: size
            }
            .rotate_n(rotation_cw);
        let edges = descr.edges.iter().map(
            |&(dir, k)| {
                let rot_dir = dir.rotate_cw_n(rotation_cw);
                let corner = rect.first_corner_cw(rot_dir);
                let perp_dir = rot_dir.rotate_cw();
                (perp_dir.apply_n(corner, k), rot_dir)
            }).collect::<Vec<(circuit::Coords, Dir)>>();
        let edge_points = edges.iter()
            .map(|&(pos, _dir)| { pos })
            .collect::<HashSet<circuit::Coords>>()
            .iter()
            .cloned()
            .collect::<Vec<circuit::Coords>>();

        Component {
            element: *self,
            pos: top_left_pos,
            rotation_cw: rotation_cw,
            rect: rect,
            edges: edges,
            edge_points: edge_points
        }
    }
}

impl Component {
    pub fn size(&self) -> circuit::Coords {
        self.rect.size
    }

    pub fn edge_point_index(&self, pos: circuit::Coords) -> Option<usize> {
        self.edge_points.iter().position(|&p| p == pos)
    }
}
