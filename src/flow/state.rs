use std::ops::Neg;

use circuit::{self, Element, Circuit, CellId};
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
    pub in_flow: usize,
    pub out_flow: usize,

    // Index in the matrix for pressure solving
    pub mut_idx: Option<usize>,
}

pub struct Component {
    pub element: Element,
    pub cells: Vec<NodeIndex>
}

pub struct State {
    pub graph: CompactGraph<CellId>,
    pub flow: CompactGraphState<Cell, Edge>,
    pub components: Vec<Component>,
    pub mut_idx_to_node_idx: Vec<NodeIndex>,
    pub source_cells: Vec<NodeIndex>,
    pub sink_cells: Vec<NodeIndex>,
}

pub fn edge_quantity<T: Neg<Output=T>>(
    from_idx: NodeIndex, to_idx: NodeIndex, q: T
) -> T {
    if from_idx < to_idx {
        q
    } else {
        -q
    }
}

impl State {
    fn new_component(
        circuit: &Circuit,
        graph: &CompactGraph<CellId>,
        component: &circuit::Component
    ) -> Component {
        let cells = component.cells.iter().map(
            |cell_pos| {
                let point = circuit.points().get(cell_pos).unwrap();
                let cell_index = point.1.unwrap();
                let cell_id = (point.0, cell_index);
                graph.node_index(cell_id)
            }).collect();
        
        Component {
            element: component.element,
            cells: cells
        }
    }
    
    pub fn update_mut_indices(&mut self) {
        let mut mut_idx_to_node_idx = Vec::new();

        for node_idx in 0 .. self.graph.num_nodes() {
            let is_mut = {
                let node = self.flow.node(node_idx);
                let any_neighbor = self.graph.neighbors(node_idx).iter().any(
                    |&(_, edge_idx)| self.flow.edge(edge_idx).enabled);
                !node.bound_pressure && any_neighbor
            };
            
            self.flow.node_mut(node_idx).mut_idx =
                if is_mut {
                    mut_idx_to_node_idx.push(node_idx);
                    Some(mut_idx_to_node_idx.len()-1)
                } else {
                    None
                };
        }
        
        self.mut_idx_to_node_idx = mut_idx_to_node_idx;
    }

    pub fn from_circuit(circuit: &Circuit) -> State {
        let mut source_cells = Vec::new();
        let mut sink_cells = Vec::new();
        
        let mut node_idx_counter = 0;
        let mut mut_idx_counter = 0;
        let mut mut_idx_to_node_idx = Vec::new();
        
        let graph = CompactGraph::new(&circuit.graph());
        let flow = CompactGraphState::new(&circuit.graph(),
            |(component_id, cell_index), _node| {
                let component =
                    circuit.components().get(&component_id).unwrap();

                // TODO: Clean up
                let res = match component.element {
                    Element::Source => {
                        let new_cell = Cell {
                            bound_pressure: true,
                            pressure: 100.0,
                            load: 0,
                            old_load: 0,
                            in_flow: 0,
                            out_flow: 0,
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
                            in_flow: 0,
                            out_flow: 0,
                            mut_idx: None,
                        };
                        sink_cells.push(node_idx_counter);
                        new_cell
                    },
                    _ => {
                        let is_mut = 
                            if let Element::Switch(_) = component.element {
                                cell_index != 0
                            } else {
                                true
                            };
                        
                        let mut_idx =
                            if is_mut {
                                mut_idx_to_node_idx.push(node_idx_counter);
                                mut_idx_counter += 1;
                                Some(mut_idx_counter-1)
                            } else {
                                None
                            };
                        
                        let new_cell = Cell {
                            bound_pressure: !is_mut,
                            pressure: 0.0,
                            load: 0,
                            old_load: 0,
                            in_flow: 0,
                            out_flow: 0,
                            mut_idx: mut_idx
                        };
                        
                        new_cell
                    },
                };
                
                // Control cell of switch is a sink
                if let Element::Switch(_) = component.element {
                    if cell_index == 0 {
                        sink_cells.push(node_idx_counter);
                    }
                }
                
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
        
        let components = circuit.components().iter().map(
            |(&id, component)| {
                State::new_component(circuit, &graph, component)
            }).collect();
        
        State {
            graph: graph,
            flow: flow,
            components: components,
            mut_idx_to_node_idx: mut_idx_to_node_idx,
            source_cells: source_cells,
            sink_cells: sink_cells,
        }
    }
}
