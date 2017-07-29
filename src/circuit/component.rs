use cgmath::Zero;

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
    Bridge,
    Switch(SwitchType),
    Source,
    Sink,
    Input { size: usize },
    Output { size: usize },
    Power
}

pub struct ElementDescr {
    // Width and height. Each element occupies a rect of grid points.
    pub size: circuit::Coords,

    // Potential input/output edges for this type of element, each described
    // by the side of the rect they are and the position on that side.
    // NOTE: edges is assumed not to contain duplicates. Also, the side 
    //       positions must be smaller than the size.
    pub cells: Vec<(Dir, usize)>,
    
    pub cell_edges: Vec<Vec<Dir>>
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
    pub cells: Vec<circuit::Coords>,
    
    pub cell_edges: Vec<Vec<Dir>>
}

impl Element {
    pub fn descr(&self) -> ElementDescr {
        let (size, cells, cell_edges) = match *self {
            Element::Node =>
                (circuit::Coords::new(0, 0),
                 vec![(Dir::Left, 0)],
                 None),
            Element::Bridge =>
                (circuit::Coords::new(0, 0),
                 vec![(Dir::Left, 0), (Dir::Left, 0)],
                 Some(vec![vec![Dir::Up, Dir::Down],
                           vec![Dir::Left, Dir::Right]])),
            Element::Switch(_) =>
                (circuit::Coords::new(1, 0),
                 vec![(Dir::Left, 0), (Dir::Right, 0)],
                 None),
            Element::Source => 
                (circuit::Coords::new(0, 0),
                 vec![(Dir::Right, 0)],
                 None),
            Element::Sink =>
                (circuit::Coords::new(0, 0),
                 vec![(Dir::Left, 0)],
                 None),
            Element::Input { size } =>
                (circuit::Coords::new(0, size as isize - 1),
                 (0..size).map(|i| (Dir::Left, i)).collect(),
                 None),
            Element::Output { size } =>
                (circuit::Coords::new(0, size as isize - 1),
                 (0..size).map(|i| (Dir::Left, i)).collect(),
                 None),
            Element::Power =>
                (circuit::Coords::new(0, 0),
                 vec![(Dir::Left, 0), (Dir::Left, 0)],
                 Some(vec![vec![Dir::Left], vec![Dir::Right]]))
        };
        
        let cell_edges = match cell_edges {
            Some(cell_edges) => cell_edges,
            None => {
                let rect =
                    Rect {
                        pos: circuit::Coords::zero(),
                        size: size
                    };
            
                cells.iter().map(
                    |&(cell_dir, cell_k)| {
                        let corner = rect.first_corner_cw(cell_dir);
                        let perp_dir = cell_dir.rotate_cw();
                        let cell_pos = perp_dir.apply_n(corner, cell_k);
                        Dir::iter().filter_map(
                            |&edge_dir| {
                                if rect.is_within(edge_dir.apply(cell_pos)) {
                                    None
                                } else {
                                    Some(edge_dir)
                                }
                            }).collect()
                    }).collect()
            }
        };
        
        ElementDescr { size, cells, cell_edges }
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
            }).collect();
        let cell_edges = descr.cell_edges.iter().map(|edge_dirs|
            edge_dirs.iter().map(|&edge_dir| {
                edge_dir.rotate_cw_n(rotation_cw)
            }).collect()).collect();

        Component {
            element: *self,
            pos: top_left_pos,
            rotation_cw,
            rect,
            cells,
            cell_edges,
        }
    }
}

impl Component {
    pub fn size(&self) -> circuit::Coords {
        self.rect.size
    }
    
    pub fn get_edge_cell_index(
        &self,
        p: circuit::Coords,
        dir: Dir
    ) -> Option<usize> {
        self.cells.iter()
            .zip(self.cell_edges.iter())
            .enumerate()
            .find(|&(_, (&cell_pos, cell_edges))| {
                    cell_pos == p &&
                    cell_edges.iter().find(|&&edge_dir| edge_dir == dir)
                              .is_some()
                  })
            .map(|(cell_index, _)| cell_index)
    }
}
