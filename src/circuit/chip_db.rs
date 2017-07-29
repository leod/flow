use std::collections::HashMap;

use cgmath::Zero;

use super::{Coords, ChipId, ChipDescr, Element, Circuit, Action};

pub struct Chip {
    descr: ChipDescr,
    circuit: Circuit,
}

pub struct ChipDb {
    chips: HashMap<ChipId, Chip>
}

impl ChipDb {
    fn new_circuit(descr: &ChipDescr) -> Circuit {
        let mut circuit = Circuit::new();
        
        {
            let element = Element::Input { size: descr.left_size };
            let pos = Coords::zero();
            let component = element.new_component(pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
        }
        {
            let element = Element::Input { size: descr.right_size };
            let pos = Coords::new(0, descr.inner_size.y);
            let component = element.new_component(pos, 0);
            let action = Action::PlaceComponent(component);
            action.perform(&mut circuit);
        }
        
        circuit
    }

    pub fn init(n_chips: usize) -> ChipDb {
        let chips = (0 .. n_chips).map(
            |i| {
                let descr = ChipDescr {
                    inner_size: Coords::new(10, 10),
                    left_size: 3,
                    right_size: 3
                };
                let circuit = Self::new_circuit(&descr);
                let chip = Chip { descr, circuit };

                (i, chip)
            }).collect();

        ChipDb { chips }
    }

    pub fn get(&self, id: ChipId) -> Option<&Chip> {
        self.chips.get(&id)        
    }

    pub fn get_descr(&self, id: ChipId) -> Option<&ChipDescr> {
        self.chips.get(&id).map(|chip| &chip.descr)
    }

    pub fn get_circuit(&self, id: ChipId) -> Option<&Circuit> {
        self.chips.get(&id).map(|chip| &chip.circuit)
    }

    pub fn get_circuit_mut(&mut self, id: ChipId) -> Option<&mut Circuit> {
        self.chips.get_mut(&id).map(|chip| &mut chip.circuit)
    }
}
