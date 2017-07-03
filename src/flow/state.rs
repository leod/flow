use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::Component;
use circuit::{self, Circuit, ComponentId};

pub type CellIndex = usize;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Connection {
    velocity: f64,
}

pub struct Cell {
    pressure: f64,
}

pub struct CellConnections {
    right: Option<(CellIndex, Connection)>,
    down: Option<(CellIndex, Connection)>,
    left: Option<CellIndex>,
    up: Option<CellIndex>,
}

pub struct ComponentCells {
    indices: Vec<CellIndex>
}

pub struct State {
    cells: Vec<Cell>,
    connections: Vec<CellConnections>,
    component_cells: HashMap<ComponentId, ComponentCells>
}

impl State {
    /*fn connection(
        &self,
        index: CellIndex, 
        dir: Dir
    ) -> Option<(CellIndex, Connection)> {
        match dir {
            Dir::Right => self.cell_connections[index].right,
            Dir::Down => self.cell_connections[index].down,
            Dir::Left =>
                self.cells[index].left.map(|other_index| (other_index,
                    self.cell_connections[other_index].right.unwrap().1)),
            Dir::Up =>
                self.cells[index].up.map(|other_index| (other_index,
                    self.cell_connections[other_index].down.unwrap().1))
        }
    }

    fn new_cell(c: &Circuit, id

    pub fn from_circuit(c: &Circuit) -> Graph {
        let mut component_cells = HashMap::<ComponentId, ComponentCells>::new();
        let cells_with_ids = c.components().iter().flat_map(
            |&(id, comp)| {
                comp.edge_points.iter().zip(repeat(id))
            }
        State {

        }
    }*/
}
