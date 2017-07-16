use num::Signed;

use circuit::{Element, Circuit, CellId};
use graph::{NodeIndex, CompactGraph, CompactGraphState};

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub resistance: f64,

    pub enabled: bool,

    // Velocity from the smaller node index to the larger node index
    pub velocity: f64,
    pub old_velocity: f64,

    // Flow in the last tick from the smaller node index to the larger node index
    pub flow: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    // Whether pressure is to be recomputed
    pub bound_pressure: bool,

    // Pressure of cell, recomputed every step
    pub pressure: f64,

    // Blobs moving through the graph
    pub load: usize,
    pub old_load: usize,

    // Index in the matrix for pressure solving
    pub mut_idx: Option<usize>,
}

pub struct State {
    pub graph: CompactGraph<CellId>,
    pub flow: CompactGraphState<Cell, Edge>,
    
    pub mut_idx_to_node_idx: Vec<NodeIndex>,
    pub source_cells: Vec<NodeIndex>,
    pub sink_cells: Vec<NodeIndex>,
}

pub fn edge_quantity<T: Signed>(
    from_idx: NodeIndex, to_idx: NodeIndex, q: T
) -> T {
    if from_idx < to_idx {
        q
    } else {
        -q
    }
}

impl State {
    pub fn from_circuit(circuit: &Circuit) -> State {
        let mut source_cells = Vec::new();
        let mut sink_cells = Vec::new();
        
        let mut node_idx_counter = 0;
        let mut mut_idx_counter = 0;
        let mut mut_idx_to_node_idx = Vec::new();
        
        let graph = CompactGraph::new(&circuit.graph());
        let flow = CompactGraphState::new(&circuit.graph(),
            |(component_id, _cell_index), _node| {
                let component =
                    circuit.components().get(&component_id).unwrap();

                let res = match component.element {
                    Element::Source => {
                        let new_cell = Cell {
                            bound_pressure: true,
                            pressure: 100.0,
                            load: 0,
                            old_load: 0,
                            mut_idx: None,
                        };
                        source_cells.push(node_idx_counter);
                        new_cell
                    },
                    Element::Sink => {
                        let new_cell = Cell {
                            bound_pressure: true,
                            pressure: 0.0,
                            load: 0,
                            old_load: 0,
                            mut_idx: None,
                        };
                        sink_cells.push(node_idx_counter);
                        new_cell
                    },
                    _ => {
                        let new_cell = Cell {
                            bound_pressure: false,
                            pressure: 0.0,
                            load: 0,
                            old_load: 0,
                            mut_idx: Some(mut_idx_counter),
                        };
                        mut_idx_to_node_idx.push(node_idx_counter);
                        mut_idx_counter += 1;
                        new_cell
                    },
                };
                node_idx_counter += 1;
                res
            },
            |_, _, _| {
                Edge {
                    resistance: 0.0,
                    enabled: true,
                    velocity: 0.0,
                    old_velocity: 0.0,
                    flow: 0
                }
            });
            
        State {
            graph: graph,
            flow: flow,
            mut_idx_to_node_idx: mut_idx_to_node_idx,
            source_cells: source_cells,
            sink_cells: sink_cells,
        }
    }
}
