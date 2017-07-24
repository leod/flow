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
    Sink,
    Input { size: usize },
    Output { size: usize }
}

pub struct ElementDescr {
    // Width and height. Each element occupies a rect of grid points.
    pub size: circuit::Coords,

    // Potential input/output edges for this type of element, each described
    // by the side of the rect they are and the position on that side.
    // NOTE: edges is assumed not to contain duplicates. Also, the side 
    //       positions must be smaller than the size.
    pub cells: Vec<(Dir, usize)>
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

    // Unique positions of cells
    pub cells: Vec<circuit::Coords>
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        match *self {
            Element::Node => ElementDescr {
                size: circuit::Coords::new(0, 0),
                cells: vec![(Dir::Left, 0)]
            },
            Element::Switch(_) => ElementDescr {
                size: circuit::Coords::new(1, 0),
                cells: vec![(Dir::Left, 0), (Dir::Right, 0)],
            },
            Element::Source => ElementDescr {
                size: circuit::Coords::new(2, 2),
                cells: vec![(Dir::Right, 1)],
            },
            Element::Sink => ElementDescr {
                size: circuit::Coords::new(0, 0),
                cells: vec![(Dir::Left, 0)]
            },
            Element::Input { size } => ElementDescr {
                size: circuit::Coords::new(0, size as isize),
                cells: (0..size).map(|i| (Dir::Left, i)).collect()
            },
            Element::Output { size } => ElementDescr {
                size: circuit::Coords::new(0, size as isize),
                cells: (0..size).map(|i| (Dir::Left, i)).collect()
            }
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
        let cells = descr.cells.iter().map(
            |&(dir, k)| {
                let rot_dir = dir.rotate_cw_n(rotation_cw);
                let corner = rect.first_corner_cw(rot_dir);
                let perp_dir = rot_dir.rotate_cw();
                perp_dir.apply_n(corner, k)
            }).collect::<Vec<circuit::Coords>>();

        Component {
            element: *self,
            pos: top_left_pos,
            rotation_cw: rotation_cw,
            rect: rect,
            cells: cells,
        }
    }
}

impl Component {
    pub fn size(&self) -> circuit::Coords {
        self.rect.size
    }
}
