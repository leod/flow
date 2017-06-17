use std::collections::HashMap;

use types::Dir;
use grid::{self, Coords, Point, Edge, EdgeMap};
use component::{ComponentId, Component};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Action {
    PlaceComponent(Component),
    RemoveComponent(ComponentId),
    RemoveComponentAtPos(Coords),
    PlaceEdge(Coords, Dir, Edge),
    RemoveEdge(Coords, Dir),
    Compound(Vec<Action>),
    ReverseCompound(Vec<Action>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

fn is_edge_component_conflict(pos: Coords, dir: Dir, comp: &Component) -> bool{
    println!("check {:?} -> {:?} with {:?}", pos, dir, comp);
    if comp.rect.is_within(pos) && comp.rect.is_within(dir.apply(pos)) {
        true
    } else if comp.rect.is_within(pos) {
        comp.edge_points.iter()
            .find(|&&(point, point_dir)|
                  grid::canonize_edge(point, point_dir) ==
                  grid::canonize_edge(pos, dir.invert()))
            .is_none()
    } else if comp.rect.is_within(dir.apply(pos)) {
        comp.edge_points.iter()
            .find(|&&(point, point_dir)|
                  grid::canonize_edge(point, point_dir) ==
                  grid::canonize_edge(pos, dir))
            .is_none()
    } else {
        false
    }
}

impl Action {
    pub fn can_perform(&self, circuit: &Circuit) -> bool {
        match self {
            &Action::PlaceComponent(ref component) => {
                // Check that the grid points are empty
                let points_empty = component.rect
                    .iter()
                    .all(|c| !circuit.points.contains_key(&c));

                // Check that existing edges are ok
                let rect = component.rect.iter();
                let edge_conflict =
                    rect.map(|pos|
                              circuit.edges().iter_dirs(pos)
                                     .map(|(dir, _edge)| 
                                          is_edge_component_conflict(pos,
                                              dir, component))
                                     .any(|b| b))
                        .any(|b| b);

                points_empty && !edge_conflict
            }
            &Action::RemoveComponent(ref component_id) => {
                circuit.components.contains_key(component_id)
            }
            &Action::RemoveComponentAtPos(pos) => {
                circuit.points.get(&pos).is_some()
            }
            &Action::PlaceEdge(pos, dir, _edge) => {
                let point_a = circuit.points.get(&pos);
                let point_b = circuit.points.get(&dir.apply(pos));
                let circuit_points = point_a.iter().chain(point_b.iter());
                let component_conflict =
                    circuit_points.map(|id| {
                        let comp = circuit.components().get(&id.0).unwrap();
                        let b = is_edge_component_conflict(pos, dir, &comp);
                        println!("{}", b);
                        b
                    }).any(|b| b);

                circuit.edges.get(pos, dir).is_none() && !component_conflict
            }
            &Action::RemoveEdge(pos, dir) => {
                circuit.edges.get(pos, dir).is_some()
            }
            &Action::Compound(_) => {
                // Compound not included here
                true
            }
            &Action::ReverseCompound(_) => {
                // ReverseCompound not included here
                true
            }
        }
    }

    // Returns an action that reverts the perfomed action
    pub fn perform(self, circuit: &mut Circuit) -> Action {
        assert!(self.can_perform(circuit));

        println!("circuit action: {:?}", self);

        match self {
            Action::PlaceComponent(ref component) => {
                // Insert in component map
                let component_id = circuit.next_component_id;
                assert!(!circuit.components.contains_key(&component_id));
                circuit.components.insert(component_id, component.clone());

                circuit.next_component_id += 1;

                // Mark grid points as used
                let point = Point(component_id);
                for c in component.rect.iter() {
                    circuit.points.insert(c, point);
                    println!("mark {:?}", c);
                }

                Action::RemoveComponent(component_id)
            }
            Action::RemoveComponent(component_id) => {
                let component = circuit.components
                    .get(&component_id).unwrap().clone();
                circuit.components.remove(&component_id); 

                for c in component.rect.iter() {
                    circuit.points.remove(&c);
                }

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
                let undo = actions.into_iter()
                                  .map(|action| { action.perform(circuit) })
                                  .collect::<Vec<_>>();
                Action::ReverseCompound(undo) 
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
