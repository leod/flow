mod action;
mod component;
mod chip_db;

use std::collections::{HashMap, HashSet};

use types::Dir;
use graph::NeighborGraph;

pub use types::Coords;
pub use self::action::Action;
pub use self::component::{SwitchType, ChipId, ChipDescr, 
    ElementDescr, Element, Component};
pub use self::chip_db::{Chip, ChipDb};

pub type ComponentId = usize;

// Each component consists of cells where edges can attach. The cells are
// created at the edge points of the component, which are described by its
// element.
pub type CellId = (ComponentId, usize);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Edge {
}

pub type Point = ComponentId;

pub type Graph = NeighborGraph<CellId, Coords, Edge>;

#[derive(Clone)]
pub struct Circuit {
    // Components of the circuit
    components: HashMap<ComponentId, Component>,

    // Cells, and edges between them.
    graph: Graph,

    // Grid coords that are occupied by components. Note that this can be
    // completely derived from the components. The point of this is to make it
    // easy for the hud to know which grid points are already in use.
    points: HashMap<Coords, Point>,

    // Counter to create unique component ids
    next_component_id: ComponentId,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit {
            components: HashMap::new(),
            graph: Graph::new(),
            points: HashMap::new(),
            next_component_id: 0,
        }
    }

    pub fn points(&self) -> &HashMap<Coords, Point> {
        &self.points
    }

    pub fn graph(&self) -> &NeighborGraph<CellId, Coords, Edge> {
        &self.graph
    }

    pub fn components(&self) -> &HashMap<ComponentId, Component> {
        &self.components
    }

    pub fn get_last_component_id(&self) -> Option<ComponentId> {
        if self.next_component_id > 0 {
            Some(self.next_component_id - 1)
        } else {
            None
        }
    }

    // Unfold this circuit by recursively instantiating chips.
    // This mean that chip components are replaced by the circuit, as it
    // is given in the chip database.
    // Note that the points map of the resulting circuit is not valid.
    // TODO: Unfolding might also be possible to do without creating an
    // intermediate circuit, while creating the CompactGraph.
    pub fn unfold(&self, chip_db: &ChipDb) -> Circuit {
        let mut unfolded_circuit = self.clone();
        let mut finished_ids = HashSet::new();

        loop {
            let chip_component_ids = unfolded_circuit.components.iter().filter(
                |&(c_id, component)|
                    match &component.element {
                        &Element::Chip(_chip_id, ref _descr) => true,
                        _ => false
                    } && !finished_ids.contains(c_id)
                ).map(|(&c_id, component)| c_id)
                 .collect::<Vec<_>>();

            if chip_component_ids.len() == 0 {
                break;
            }

            for &chip_component_id in chip_component_ids.iter() {
                finished_ids.insert(chip_component_id);

                let chip_component = 
                    unfolded_circuit.components.get(&chip_component_id).unwrap().clone();
                let chip_element_descr = chip_component.element.descr();

                if let Element::Chip(chip_id, descr) = chip_component.element {
                    let chip = chip_db.get(&chip_id).unwrap();

                    // Map from cell IDs inside chip circuit to cell IDs in 
                    // unfolded circuit
                    let mut id_map = HashMap::new();

                    for (&c_id, component) in chip.circuit.components.iter() {
                        if c_id == chip.left_input_id {
                            // The input components of a chip serve as the gluing points
                            // to the outer circuit. Here, we can assume that the number
                            // of cell edges to the left is the same for the chip component
                            // as well as the left input inside the chip circuit.
                            let left_cells = chip_element_descr.cells.iter().enumerate()
                                .filter( |&(_i, &(dir, _k))| dir == Dir::Left)
                                .map(|(i, _)| i);
                                
                            for (inner_cell_index, cell_index) in left_cells.enumerate() {
                                id_map.insert((c_id, inner_cell_index),
                                              (chip_component_id, cell_index));
                            }
                        } else if c_id == chip.right_input_id {
                            let right_cells = chip_element_descr.cells.iter().enumerate()
                                .filter(|&(_i, &(dir, _k))| dir == Dir::Right)
                                .map(|(i, _)| i);
                                
                            for (inner_cell_index, cell_index) in right_cells.enumerate() {
                                id_map.insert((c_id, inner_cell_index),
                                              (chip_component_id, cell_index));
                            }
                        } else {
                            // Add components of inner chip 
                            let new_id = unfolded_circuit.next_component_id;
                            unfolded_circuit.next_component_id += 1;

                            for cell_index in 0 .. component.cells.len() {
                                let node = chip.circuit.graph.get_node((c_id, cell_index)).unwrap();
                                unfolded_circuit.graph.add_node((new_id, cell_index), *node);
                                id_map.insert((c_id, cell_index), (new_id, cell_index));
                            }
                            
                            unfolded_circuit.components.insert(new_id, component.clone());
                        }
                    }

                    // Insert new edges for the unfolded circuit
                    for (&(ref cell_id_a, ref cell_id_b), &edge) in chip.circuit.graph.edges().iter() {
                        let new_cell_id_a = *id_map.get(cell_id_a).unwrap();
                        let new_cell_id_b = *id_map.get(cell_id_b).unwrap();

                        unfolded_circuit.graph.add_edge(new_cell_id_a, new_cell_id_b, edge);
                    }
                } else {
                    panic!("component should be a Chip");
                }
            }
        }
        unfolded_circuit
    }
}
