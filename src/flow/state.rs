use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::Component;
use circuit::{self, Element, Circuit, ComponentId};

pub type CellIndex = usize;
pub type ConnectionIndex = usize;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Connection {
    enabled: bool,
    velocity: f64,
    resistance: f64,

    a: CellIndex,
    b: CellIndex
}

pub struct Cell {
    // whether pressure has to be recomputed
    bound_pressure: bool,

    // pressure of cell, is recomputed every step
    pressure: f64,

    // connections, static
    connections: Vec<ConnectionIndex>
}

/* maybe needed later
pub struct ComponentCells {
    indices: Vec<CellIndex>
}
*/

pub struct State {
    cells: Vec<Cell>,
    connections: Vec<Connection>,
    //component_cells: HashMap<ComponentId, ComponentCells>
}

impl State {
    pub fn from_circuit(circuit: &Circuit) -> State {
        let mut cells = circuit.components().iter().flat_map(
            |(id, c)| {
                let bound_pressure = match c.element {
                    Element::Source | Element::Sink => true,
                    _ => false
                };
                let pressure = match c.element {
                    Element::Source => 100.0,
                    _ => 0.0
                };
                vec![Cell {
                    bound_pressure: bound_pressure,
                    pressure: pressure,
                    connections: vec![]
                }]
            }).collect();
        let mut connections = circuit.components().iter().flat_map(
            |(id, c)| {
                //c.edges
                vec![] // TODO
            }).collect();
        //let mut connections = circuit.compen; TODO
        State {
            cells: cells, 
            connections: connections
        }
    }
    
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
