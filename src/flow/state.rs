use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::{self, Element, Component, Circuit, ComponentId, CellId};

use graph::{NodeIndex, GraphState};

#[derive(Clone, Copy, Debug)]
pub struct Connection {
    pub enabled: bool,
    pub velocity: f64,
    pub old_velocity: f64,
    pub resistance: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    // Whether pressure is to be recomputed
    pub bound_pressure: bool,

    // Pressure of cell, recomputed every step
    pub pressure: f64,

    // Throughput
    pub throughput: f64,
    pub old_throughput: f64,

    // for the matrix indices
    pub mut_idx: Option<usize>,
}

pub struct State {
    pub graph: GraphState<CellId, Cell, Connection>,
    pub mut_idx_to_node_idx: Vec<NodeIndex>,
}

impl State {
    pub fn from_circuit(circuit: &Circuit) -> State {
        let mut node_idx_counter = 0;
        let mut mut_idx_counter = 0;
        let mut mut_idx_to_node_idx = Vec::new();
        let graph = GraphState::new(circuit.graph(),
            |(component_id, cell_index), node| {
                let component =
                    circuit.components().get(&component_id).unwrap();

                let res = match component.element {
                    Element::Source =>
                        Cell {
                            bound_pressure: true,
                            pressure: 100.0,
                            throughput: 0.0,
                            old_throughput: 0.0,
                            mut_idx: None,
                        },
                    Element::Sink =>
                        Cell {
                            bound_pressure: true,
                            pressure: 0.0,
                            throughput: 0.0,
                            old_throughput: 0.0,
                            mut_idx: None,
                        },
                    _ => {
                        let new_cell = Cell {
                            bound_pressure: false,
                            pressure: 0.0,
                            throughput: 0.0,
                            old_throughput: 0.0,
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
                Connection {
                    enabled: true,
                    velocity: 0.0,
                    old_velocity: 0.0,
                    resistance: 0.0,
                }
            });
            
        State {
            graph: graph,
            mut_idx_to_node_idx: mut_idx_to_node_idx,
        }
    }
}
