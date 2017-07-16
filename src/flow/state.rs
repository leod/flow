use circuit::{Element, Circuit, CellId};

use graph::{NodeIndex, CompactGraph, CompactGraphState};

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub enabled: bool,
    pub velocity: f64,
    pub old_velocity: f64,
    pub resistance: f64,
    pub flow: usize,
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

    // for the matrix indices
    pub mut_idx: Option<usize>,
}

pub struct State {
    pub graph: CompactGraph<CellId>,
    pub flow: CompactGraphState<Cell, Edge>,
    
    pub mut_idx_to_node_idx: Vec<NodeIndex>,
    pub source_cells: Vec<NodeIndex>,
    pub sink_cells: Vec<NodeIndex>,
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
                    enabled: true,
                    velocity: 0.0,
                    old_velocity: 0.0,
                    resistance: 0.0,
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
