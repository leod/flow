use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::{self, Element, Component, Circuit, ComponentId, CellId};

use graph::GraphState;

pub type CellIndex = usize;
pub type ConnectionIndex = usize;

#[derive(Clone, Copy, Debug)]
pub struct Connection {
    enabled: bool,
    velocity: f64,
    resistance: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    // Whether pressure is to be recomputed
    bound_pressure: bool,

    // Pressure of cell, recomputed every step
    pressure: f64,
}

pub struct State {
    pub graph: GraphState<CellId, Cell, Connection>
}

impl State {
    pub fn from_circuit(circuit: &Circuit) -> State {
        let graph = GraphState::new(circuit.graph(),
            |(component_id, cell_index), node| {
                let component =
                    circuit.components().get(&component_id).unwrap();

                match component.element {
                    Element::Source =>
                        Cell {
                            bound_pressure: true,
                            pressure: 100.0
                        },
                    _ =>
                        Cell {
                            bound_pressure: false,
                            pressure: 0.0
                        }
                }
            },
            |_, _, _| {
                Connection {
                    enabled: true,
                    velocity: 0.0,
                    resistance: 0.0
                }
            });
            
        State {
            graph: graph
        }
    }
}
