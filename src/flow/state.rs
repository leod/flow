use std::iter::repeat;
use std::collections::HashMap;

use types::Dir;
use canon_map::CanonMap;

use circuit::Component;
use circuit::{self, Element, Circuit, ComponentId, CellId};

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
    graph: GraphState<CellId, Cell, Connection>
}
