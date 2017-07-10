use types::Dir;

use super::{Coords, Component, Edge, Circuit};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Action {
    None,
    NoUndo(Box<Action>),
    PlaceComponent(Component),
    RemoveComponentAtPos(Coords),
    PlaceEdgeAtPos(Coords, Dir, Edge),
    RemoveEdgeAtPos(Coords, Dir),
    ReverseCompound(Vec<Action>),
}

impl Action {
    pub fn can_perform(&self, circuit: &Circuit) -> bool {
        match self {
            &Action::None => true,
            &Action::NoUndo(ref action) => action.can_perform(circuit),
            &Action::PlaceComponent(ref component) => {
                // Check that the grid points are empty
                let points_empty = component.rect
                    .iter()
                    .all(|p| !circuit.points.contains_key(&p));

                points_empty
            }
            &Action::RemoveComponentAtPos(pos) => {
                circuit.points.get(&pos).is_some()
            }
            &Action::PlaceEdgeAtPos(pos, dir, _edge) => {
                let point_a = circuit.points.get(&pos);
                let point_b = circuit.points.get(&dir.apply(pos));

                match (point_a, point_b) {
                    (Some(&(id_a, Some(cell_a))),
                     Some(&(id_b, Some(cell_b)))) => {
                        id_a != id_b && 
                            circuit.graph.get_edge((id_a, cell_a),
                                                   (id_b, cell_b)).is_none()
                    }
                    _ => false
                }
            }
            &Action::RemoveEdgeAtPos(pos, dir) => {
                let point_a = circuit.points.get(&pos);
                let point_b = circuit.points.get(&dir.apply(pos));
                
                match (point_a, point_b) {
                    (Some(&(c_a, Some(cell_a))),
                     Some(&(c_b, Some(cell_b)))) => {
                        let id_a = (c_a, cell_a);
                        let id_b = (c_b, cell_b);
                        circuit.graph.get_edge(id_a, id_b).is_some()
                    }
                    _ => false
                }
            }
            &Action::ReverseCompound(_) => {
                // ReverseCompound not included here
                true
            }
        }
    }

    // Returns an action that reverts the perfomed action
    pub fn perform(self, circuit: &mut Circuit) -> Action {
        println!("circuit action: {:?}", self);

        assert!(self.can_perform(circuit));

        match self {
            Action::None => Action::None,
            Action::NoUndo(action) => {
                action.perform(circuit);
                Action::None
            }
            Action::PlaceComponent(ref component) => {
                // Insert in component map
                let component_id = circuit.next_component_id;
                assert!(!circuit.components.contains_key(&component_id));
                circuit.components.insert(component_id, component.clone());

                circuit.next_component_id += 1;

                // Create cells in graph
                for (i, &pos) in component.cells.iter().enumerate() {
                    circuit.graph.add_node((component_id, i), pos);
                }

                // Mark grid points as used
                for c in component.rect.iter() {
                    let cell_index = component.cells.iter().position(|&x| x == c);
                    circuit.points.insert(c, (component_id, cell_index));
                    println!("mark {:?}", c);
                }

                Action::RemoveComponentAtPos(component.pos)
            }
            Action::RemoveComponentAtPos(pos) => {
                let (component_id, _) = *circuit.points.get(&pos).unwrap();

                let component = circuit.components
                    .get(&component_id).unwrap().clone();

                circuit.components.remove(&component_id); 

                for c in component.rect.iter() {
                    circuit.points.remove(&c);
                }

                let mut undo = Vec::new();

                // Remove cells belonging to the component
                let cells = component.cells.iter().enumerate();
                for (cell_index, &cell_pos) in cells {
                    let cell_id = (component_id, cell_index);

                    // Create undo action for the edges attached to this cell
                    let neighbor_ids = circuit.graph.get_neighbors(cell_id)
                        .unwrap().clone();
                    for &neighbor_id in neighbor_ids.iter() {
                        let neighbor_pos =
                            *circuit.graph.get_node(neighbor_id).unwrap();
                        //println!("{:?} to {:?}", cell_pos, neighbor_pos);
                        let dir = Dir::from_coords(cell_pos, neighbor_pos);
                        let edge =
                            circuit.graph.get_edge(cell_id, neighbor_id).unwrap();

                        let action = Action::NoUndo(Box::new(
                            Action::PlaceEdgeAtPos(cell_pos, dir, edge.clone())));
                        undo.push(action);
                    }

                    circuit.graph.remove_node(cell_id);
                }

                undo.push(Action::PlaceComponent(component.clone()));

                Action::ReverseCompound(undo)
            }
            Action::PlaceEdgeAtPos(pos, dir, edge) => {
                let (c_a, i_a) = *circuit.points.get(&pos).unwrap();
                let (c_b, i_b) = *circuit.points.get(&dir.apply(pos)).unwrap();

                circuit.graph.add_edge((c_a, i_a.unwrap()), 
                                       (c_b, i_b.unwrap()),
                                       edge);

                Action::RemoveEdgeAtPos(pos, dir)
            }
            Action::RemoveEdgeAtPos(pos, dir) => {
                let (c_a, i_a) = *circuit.points.get(&pos).unwrap();
                let (c_b, i_b) = *circuit.points.get(&dir.apply(pos)).unwrap();

                let edge = circuit.graph.remove_edge((c_a, i_a.unwrap()),
                                                     (c_b, i_b.unwrap()));

                Action::PlaceEdgeAtPos(pos, dir, edge)
            }
            Action::ReverseCompound(actions) => {
                let undo = actions.into_iter().rev()
                                  .map(|action| { action.perform(circuit) })
                                  .collect::<Vec<_>>();
                Action::ReverseCompound(undo) 
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
