pub use types::ComponentId;
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

    // Potential input/output edges for this type of element, each described
    // by the side of the rect they are and the position on that side.
    // NOTE: edge_points is assumed not to contain duplicates. Also, the side 
    //       positions must be smaller than the size.
    pub edge_points: Vec<(Dir, usize)>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Component {
    pub element: Element,

    // Position of the top left corner
    pub pos: circuit::Coords,

    pub rotation_cw: usize,

    // Derived quantities:
    pub rect: Rect,
    pub edge_points: Vec<(circuit::Coords, Dir)>,
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        match *self {
            Element::Node => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edge_points: vec![(Dir::Left, 0), (Dir::Right, 0),
                                  (Dir::Up, 0), (Dir::Down, 0)],
            },
            Element::Switch(_) => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edge_points: vec![(Dir::Left, 0), (Dir::Up, 0), (Dir::Down, 0)],
            },
            Element::Source => ElementDescr {
                size: circuit::Coords::new(2, 2),
                edge_points: vec![(Dir::Right, 1)],
            },
            Element::Sink => ElementDescr {
                size: circuit::Coords::new(0, 0),
                edge_points: vec![(Dir::Right, 0)]
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
        let edge_points = descr.edge_points.iter().map(
            |&(dir, k)| {
                let rot_dir = dir.rotate_cw_n(rotation_cw);
                let corner = rect.first_corner_cw(rot_dir);
                let perp_dir = rot_dir.rotate_cw();
                (perp_dir.apply_n(corner, k), rot_dir)
            }).collect();

        Component {
            element: *self,
            pos: top_left_pos,
            rotation_cw: rotation_cw,
            rect: rect,
            edge_points: edge_points
        }
    }
}

impl Component {
    pub fn size(&self) -> circuit::Coords {
        self.rect.size
    }
}
