use std::ops::Deref;

use circuit::{self, Circuit, Element, Action};
use flow;

pub struct Level {
    pub input_size: usize,
    pub input_pos: circuit::Coords,
    pub output_size: usize,
    pub output_pos: circuit::Coords,
    pub create_impl: Box<Fn() -> Box<LevelImpl>>
}

pub enum Outcome {
    Success,
    Failure
}

pub trait LevelImpl {
    fn time_step(&mut self, flow: &flow::State) -> Option<Outcome>;
}

pub struct LevelState {
    pub flow: flow::State,
    level_impl: Box<LevelImpl>,
}

impl Level {
    pub fn new_circuit(&self) -> Circuit {
        let mut circuit = Circuit::new();
        
        {
            let element = Element::Input { size: self.input_size };
            let component = element.new_component(self.input_pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
        }
        {
            let element = Element::Output { size: self.output_size };
            let component = element.new_component(self.output_pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
        }
        
        circuit
    }
}

impl Level {
    pub fn new_state(&self, circuit: &Circuit) -> LevelState {
        LevelState {
            flow: flow::State::from_circuit(circuit),
            level_impl: self.create_impl.deref()()
        }
    }
}

impl LevelState {
    pub fn time_step(&mut self) -> Option<Outcome> {
        flow::time_step(&mut self.flow, 0.0);
        self.level_impl.time_step(&self.flow)
    }
}