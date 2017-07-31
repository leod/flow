use std::collections::HashMap;

use types::Dir;

use super::{Coords, CellId, Element, Component, Edge, Circuit};

#[derive(Clone)]
pub enum Action {
    None,
    NoUndo(Box<Action>),
    PlaceComponent(Component),
    RemoveComponentAtPos(Coords),
    PlaceEdgeAtPos(Coords, Dir, Option<Edge>),
    PlaceEdge(CellId, CellId, Edge),
    RemoveEdge(CellId, CellId),
    PlaceCircuitAtPos(Circuit, Coords),
    ReverseCompound(Vec<Action>),
}

impl Action {
    pub fn can_perform(&self, circuit: &Circuit) -> bool {
        match self {
            &Action::None => true,
            &Action::NoUndo(ref action) => action.can_perform(circuit),
            &Action::PlaceComponent(ref component) => {
                // Check that the grid points are empty
                let points_empty = component.rect.iter().all(|p| {
                    !circuit.points.contains_key(&p)
                });

                points_empty
            }
            &Action::RemoveComponentAtPos(pos) => {
                let point = circuit.points.get(&pos);

                if let Some(component_id) = point {
                    let element =
                        &circuit.components.get(&component_id).unwrap().element;
                    match element {
                        &Element::Input { .. } => false,
                        &Element::Output { .. } => false,
                        _ => true,
                    }
                } else {
                    false
                }
            }
            &Action::PlaceEdgeAtPos(pos, dir, edge) => {
                let pos_b = dir.apply(pos);
                let id_a = circuit.points.get(&pos);
                let id_b = circuit.points.get(&pos_b);

                match (id_a, id_b) {
                    (Some(&id_a), Some(&id_b)) => {
                        let c_a = circuit.components.get(&id_a).unwrap();
                        let c_b = circuit.components.get(&id_b).unwrap();
                        let cell_a = c_a.get_edge_cell_index(pos, dir);
                        let cell_b =
                            c_b.get_edge_cell_index(pos_b, dir.invert());

                        match (cell_a, cell_b) {
                            (Some(cell_a), Some(cell_b)) => {
                                circuit
                                    .graph
                                    .get_edge((id_a, cell_a), (id_b, cell_b))
                                    .is_none() !=
                                    edge.is_none()
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            &Action::PlaceEdge((id_a, cell_a), (id_b, cell_b), _edge) => {
                let c_a = circuit.components.get(&id_a);
                let c_b = circuit.components.get(&id_b);

                match (c_a, c_b) {
                    (Some(_), Some(_)) => {
                        // TODO: Check that edge is contained in cell_edges
                        circuit
                            .graph
                            .get_edge((id_a, cell_a), (id_b, cell_b))
                            .is_none()
                    }
                    _ => false,
                }
            }
            &Action::RemoveEdge((id_a, cell_a), (id_b, cell_b)) => {
                let c_a = circuit.components.get(&id_a);
                let c_b = circuit.components.get(&id_b);

                match (c_a, c_b) {
                    (Some(_), Some(_)) => {
                        circuit
                            .graph
                            .get_edge((id_a, cell_a), (id_b, cell_b))
                            .is_some()
                    }
                    _ => false,
                }
            }
            &Action::PlaceCircuitAtPos(ref place_circuit, at_pos) => {
                place_circuit.points.keys().all(|&p| {
                    //println!("check {:?}: {:?} + {:?}", p + at_pos, p, at_pos);
                    !circuit.points.contains_key(&(p + at_pos))
                })
            }
            &Action::ReverseCompound(_) => {
                // ReverseCompound not included here
                true
            }
        }
    }

    // Returns an action that reverts the perfomed action
    pub fn perform(self, circuit: &mut Circuit) -> Action {
        //println!("circuit action: {:?}", self);

        //assert!(self.can_perform(circuit));

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
                    circuit.points.insert(c, component_id);
                }

                Action::RemoveComponentAtPos(component.pos)
            }
            Action::RemoveComponentAtPos(pos) => {
                let component_id = *circuit.points.get(&pos).unwrap();

                let component =
                    circuit.components.get(&component_id).unwrap().clone();

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
                    let neighbor_ids =
                        circuit.graph.get_neighbors(cell_id).unwrap().clone();
                    for &neighbor_id in neighbor_ids.iter() {
                        let neighbor_pos =
                            *circuit.graph.get_node(neighbor_id).unwrap();
                        let dir = Dir::from_coords(cell_pos, neighbor_pos);
                        let edge = circuit
                            .graph
                            .get_edge(cell_id, neighbor_id)
                            .unwrap();

                        let action =
                            Action::NoUndo(Box::new(Action::PlaceEdgeAtPos(
                                cell_pos,
                                dir,
                                Some(edge.clone()),
                            )));
                        undo.push(action);
                    }

                    circuit.graph.remove_node(cell_id);
                }

                undo.push(Action::PlaceComponent(component.clone()));

                Action::ReverseCompound(undo)
            }
            Action::PlaceEdgeAtPos(pos, dir, edge) => {
                let pos_b = dir.apply(pos);
                let id_a = *circuit.points.get(&pos).unwrap();
                let id_b = *circuit.points.get(&pos_b).unwrap();
                let (cell_a, cell_b) = {
                    let c_a = circuit.components.get(&id_a).unwrap();
                    let c_b = circuit.components.get(&id_b).unwrap();

                    (
                        c_a.get_edge_cell_index(pos, dir).unwrap(),
                        c_b.get_edge_cell_index(pos_b, dir.invert()).unwrap(),
                    )
                };
                let node_a = (id_a, cell_a);
                let node_b = (id_b, cell_b);

                match edge {
                    Some(edge) => {
                        circuit.graph.add_edge(node_a, node_b, edge);
                        Action::PlaceEdgeAtPos(pos, dir, None)
                    }
                    None => {
                        let edge = circuit.graph.remove_edge(node_a, node_b);
                        Action::PlaceEdgeAtPos(pos, dir, Some(edge))
                    }
                }
            }
            Action::PlaceEdge((id_a, cell_a), (id_b, cell_b), edge) => {
                circuit.graph.add_edge((id_a, cell_a), (id_b, cell_b), edge);
                Action::RemoveEdge((id_a, cell_a), (id_b, cell_b))
            }
            Action::RemoveEdge((id_a, cell_a), (id_b, cell_b)) => {
                let edge =
                    circuit.graph.remove_edge((id_a, cell_a), (id_b, cell_b));
                Action::PlaceEdge((id_a, cell_a), (id_b, cell_b), edge)
            }
            Action::PlaceCircuitAtPos(place_circuit, at_pos) => {
                let result = place_circuit
                    .components
                    .iter()
                    .map(|(&component_id, place_component)| {
                        let mut new_component = place_component.clone();
                        new_component.pos += at_pos;
                        new_component.rect.pos += at_pos;
                        for cell_pos in new_component.cells.iter_mut() {
                            *cell_pos += at_pos;
                        }

                        let undo = Action::PlaceComponent(new_component)
                            .perform(circuit);
                        let new_component_id =
                            circuit.get_last_component_id().unwrap();
                        (undo, (component_id, new_component_id))
                    })
                    .collect::<Vec<_>>();
                let undo =
                    result.iter().map(|&(ref undo, _)| undo.clone()).collect();
                let id_map =
                    result.iter().map(|&(_, m)| m).collect::<HashMap<_, _>>();
                for (&((id_a, cell_a), (id_b, cell_b)), edge) in
                    place_circuit.graph.edges.iter()
                {
                    let new_id_a = *id_map.get(&id_a).unwrap();
                    let new_id_b = *id_map.get(&id_b).unwrap();
                    circuit.graph.add_edge(
                        (new_id_a, cell_a),
                        (new_id_b, cell_b),
                        edge.clone(),
                    );
                }
                Action::ReverseCompound(undo)
            }
            Action::ReverseCompound(actions) => {
                let undo = actions
                    .into_iter()
                    .rev()
                    .map(|action| action.perform(circuit))
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
