use std::collections::HashMap;

use types::Dir;
use types::Rect;
use grid::{Coords, Point, Edge, EdgeMap};
use component::{self, ComponentId, Element, Component};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Action {
    PlaceComponent(Component),
    RemoveComponent(ComponentId),
    RemoveComponentAtPos(Coords),
    PlaceEdge(Coords, Dir, Edge),
    RemoveEdge(Coords, Dir),
    Compound(Vec<Action>),
}

pub struct Circuit {
    points: HashMap<Coords, Point>,
    edges: EdgeMap<Edge>,

    components: HashMap<ComponentId, Component>,
    next_component_id: ComponentId,
}

impl Circuit {
    pub fn empty() -> Circuit {
        Circuit {
            points: HashMap::new(),
            edges: EdgeMap::new(),
            components: HashMap::new(),
            next_component_id: 0,
        }
    }

    pub fn points(&self) -> &HashMap<Coords, Point> {
        &self.points
    }

    pub fn edges(&self) -> &EdgeMap<Edge> {
        &self.edges
    }

    pub fn components(&self) -> &HashMap<ComponentId, Component> {
        &self.components
    }

    /*pub fn new(edges: EdgeMap<Edge>, cs: HashMap<ComponentId, Component>) -> Circuit {
        Circuit {
            grid: grid,
            components: cs,
            next_component_id: cs.keys().max() + 1,
        }
    }*/
}

impl Action {
    pub fn can_perform(&self, circuit: &Circuit) -> bool {
        match self {
            &Action::PlaceComponent(ref component) => {
                let descr = component.element.descr();
                let rect = Rect {
                    pos: component.top_left_pos, 
                    size: descr.size
                };
                let rot_rect = rect.rotate_n(component.rotation);

                // Check that the grid points are empty
                rot_rect.iter().all(|c| !circuit.points.contains_key(&c))
            }
            &Action::RemoveComponent(ref component_id) => {
                circuit.components.contains_key(component_id)
            }
            &Action::RemoveComponentAtPos(pos) => {
                circuit.points.get(&pos).is_some()
            }
            &Action::PlaceEdge(pos, dir, edge) => {
                // Check that we are not trying to place an edge in the middle
                // of a component
                let in_component = 
                    match (circuit.points.get(&pos),
                           circuit.points.get(&dir.apply(pos))) {
                        (Some(id1), Some(id2)) => id1 == id2,
                        _ => false
                    };

                circuit.edges.get(pos, dir).is_none() && !in_component
            }
            &Action::RemoveEdge(pos, dir) => {
                circuit.edges.get(pos, dir).is_some()
            }
            &Action::Compound(_) => {
                // Compound not included here
                true
            }
        }
    }

    // Returns an action that reverts the perfomed action
    pub fn perform(self, circuit: &mut Circuit) -> Action {
        assert!(self.can_perform(circuit));

        match self {
            Action::PlaceComponent(ref component) => {
                // Insert in component map
                let component_id = circuit.next_component_id;
                assert!(!circuit.components.contains_key(&component_id));
                circuit.components.insert(component_id, component.clone());

                circuit.next_component_id += 1;

                // Mark grid points as used
                let descr = component.element.descr();
                let rect = Rect {
                    pos: component.top_left_pos, 
                    size: descr.size
                };
                let rot_rect = rect.rotate_n(component.rotation);

                let point = Point(component_id);

                for c in rot_rect.iter() {
                    circuit.points.insert(c, point);
                }

                Action::RemoveComponent(component_id)
            }
            Action::RemoveComponent(component_id) => {
                let component = circuit.components
                    .get(&component_id).unwrap().clone();
                circuit.components.remove(&component_id); 

                Action::PlaceComponent(component.clone())
            }
            Action::RemoveComponentAtPos(pos) => {
                let Point(component_id) = *circuit.points.get(&pos).unwrap();
                Action::RemoveComponent(component_id).perform(circuit)
            }
            Action::PlaceEdge(c, dir, edge) => {
                circuit.edges.set(c, dir, edge);

                Action::RemoveEdge(c, dir)
            }
            Action::RemoveEdge(c, dir) => {
                let edge = *circuit.edges.get(c, dir).unwrap();
                circuit.edges.remove(c, dir);

                Action::PlaceEdge(c, dir, edge)
            }
            Action::Compound(actions) => {
                let mut undo = actions.into_iter()
                                      .map(|action| { action.perform(circuit) })
                                      .collect::<Vec<_>>();
                undo.reverse();
                Action::Compound(undo) 
            }
        }
    }

    pub fn try_perform(self, circuit: &mut Circuit) -> Option<Action> {
        if self.can_perform(circuit) {
            Some(self.perform(circuit))
        } else {
            None
        }
    }
}
