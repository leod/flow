use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::Component;
use circuit::{self, Element, Circuit, ComponentId, EdgeMap};

pub type CellIndex = usize;
pub type ConnectionIndex = usize;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Connection {
    enabled: bool,
    velocity: f64,
    resistance: f64,

    index_a: CellIndex,
    index_b: CellIndex
}

pub struct Cell {
    // whether pressure has to be recomputed
    bound_pressure: bool,

    // pressure of cell, is recomputed every step
    pressure: f64,

    // connections, static
    neighbors: Vec<ConnectionIndex>
}

pub struct State {
    cells: Vec<Cell>,
    connections: Vec<Connection>,

    // Cells of each component in the circuit.
    // Same order as the component's edge points.
    // TODO: Move to separate struct
    component_cell_indices: HashMap<ComponentId, Vec<CellIndex>>,
    connection_indices: CanonMap<(circuit::Coords, Dir), ConnectionIndex>
}

impl State {
    pub fn get_cell(&self, id: ComponentId, edge_point_index: usize) -> &Cell {
        let index =
            self.component_cell_indices.get(&id).unwrap()[edge_point_index];
        &self.cells[index]
    }

    pub fn get_connection(&self, p: circuit::Coords, dir: Dir) -> &Connection {
       let index
           = *self.connection_indices.get((p, dir)).unwrap();
       &self.connections[index]
    }

    pub fn from_circuit(circuit: &Circuit) -> State {
        // Create component cells and map of indices
        let mut cells = Vec::new();
        let mut component_cell_indices = Vec::new();

        for (&id, component) in circuit.components().iter() {
            let bound_pressure = match component.element {
                Element::Source | Element::Sink => true,
                _ => false
            };
            let pressure = match component.element {
                Element::Source => 100.0,
                _ => 0.0
            };

            let mut cell_indices = Vec::new();
            
            for p in component.edge_points.iter() {
                let cell = Cell {
                    bound_pressure: bound_pressure,
                    pressure: pressure,
                    neighbors: vec![]
                };

                cell_indices.push(cells.len());
                cells.push(cell);
            }

            component_cell_indices.push((id, cell_indices));
        }
        let component_cell_indices = component_cell_indices
            .iter().cloned().collect::<HashMap<_, _>>();

        // Create connections and store in cells
        let mut connections = Vec::new(); 
        let mut connection_indices = CanonMap::new();

        for (id_a, component_a) in circuit.components().iter() {
            for &(pos_a, dir) in component_a.edges.iter() {
                if circuit.edges().get((pos_a, dir)).is_some() {
                    let pos_b = dir.apply(pos_a);

                    // Every edge is always attached to two components,
                    // find the second one
                    let id_b = circuit.points().get(&pos_b).unwrap().0;
                    let component_b = circuit.components().get(&id_b).unwrap();

                    // Get the corresponding cell index
                    let index_a = component_cell_indices.get(&id_a).unwrap()
                        [component_a.edge_point_index(pos_a).unwrap()];
                    let index_b = component_cell_indices.get(&id_b).unwrap()
                        [component_b.edge_point_index(pos_b).unwrap()];

                    // Add connection to both cells neighbors
                    let connection_index = connections.len();

                    cells[index_a].neighbors.push(index_a);
                    cells[index_b].neighbors.push(index_b);

                    let connection = Connection {
                        enabled: true, 
                        velocity: 0.0,
                        resistance: 0.0,
                        index_a: index_a,
                        index_b: index_b,
                    };

                    connections.push(connection);

                    connection_indices.set((pos_a, dir), connection_index);
                }
            }
        }

        State {
            cells: cells, 
            connections: connections,

            component_cell_indices: component_cell_indices,
            connection_indices: connection_indices
        }
    }
}
