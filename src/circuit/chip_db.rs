use std::collections::HashMap;

use cgmath::Zero;

use super::{Coords, ComponentId, ChipId, ChipDescr, Element, Circuit, Action};

pub struct Chip {
    pub descr: ChipDescr,
    pub circuit: Circuit,
    pub left_input_id: ComponentId,
    pub right_input_id: ComponentId,
}

pub struct ChipDb {
    chips: HashMap<ChipId, Chip>,
}

impl ChipDb {
    fn new_circuit(descr: &ChipDescr) -> (Circuit, ComponentId, ComponentId) {
        let mut circuit = Circuit::new();

        let left_id = {
            let element = Element::Input { size: descr.left_size };
            let pos = Coords::zero();
            let component = element.new_component(pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
            circuit.get_last_component_id().unwrap()
        };
        let right_id = {
            let element = Element::Input { size: descr.right_size };
            let pos = Coords::new(descr.inner_size.x, 0);
            let component = element.new_component(pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
            circuit.get_last_component_id().unwrap()
        };

        (circuit, left_id, right_id)
    }

    pub fn init(n_chips: usize) -> ChipDb {
        let chips = (2..2 + n_chips)
            .map(|i| {
                let descr = ChipDescr {
                    inner_size: Coords::new(10, 10),
                    left_size: 2 + i % 2,
                    right_size: 2 + i % 2,
                };
                let (circuit, left_input_id, right_input_id) =
                    Self::new_circuit(&descr);
                let chip = Chip {
                    descr,
                    circuit,
                    left_input_id,
                    right_input_id,
                };

                (i, chip)
            })
            .collect();

        ChipDb { chips }
    }

    pub fn get(&self, id: &ChipId) -> Option<&Chip> {
        self.chips.get(id)
    }

    pub fn get_descr(&self, id: &ChipId) -> Option<&ChipDescr> {
        self.chips.get(id).map(|chip| &chip.descr)
    }

    pub fn get_circuit(&self, id: &ChipId) -> Option<&Circuit> {
        self.chips.get(id).map(|chip| &chip.circuit)
    }

    pub fn get_circuit_mut(&mut self, id: &ChipId) -> Option<&mut Circuit> {
        self.chips.get_mut(id).map(|chip| &mut chip.circuit)
    }
}
