use std::collections::HashMap;

use canon_map::{Canonize, CanonMap};

pub struct Graph<NodeId: Eq + Ord + Hash, Node, Edge> {
    nodes: HashMap<NodeId, (Node, Vec<NodeId>)>,
    edges: CanonMap<(NodeId, NodeId), Edge>
}

type NeighboredGraph<NodeId: Eq + Ord + Hash, Node, Edge> =
    Graph<NodeId, (Node, Vec<NodeId>), Edge>;

type NodeIndex = usize;
type EdgeIndex = usize;

pub struct GraphState<NodeId: Eq + Ord + Hash,
                      Node, Edge,
                      NodeState, EdgeState> {
    indices: Graph<NodeId, NodeIndex, EdgeIndex>,

    nodes: Vec<(Node, Vec<EdgeIndex>)>,
    edges: Vec<Edge>
}

impl<NodeId: Eq + Ord + Hash + Copy, Node, Edge>
    NeighboredGraph<NodeId, Node, Edge> {
    pub fn new() {
        Graph {
            edges: CanonMap::new(),
            nodes: HashMap::new()
        }
    }

    pub fn add_node(&self, id: NodeId, node: Node) {
        assert!(!self.nodes.get(&id).is_some());

        self.nodes.insert(id, (node, Vec::New()));
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id).map(|(node, _)| node)
    }

    pub fn get_neighbors(&self, id: NodeId) -> &[NodeId] {
        self.nodes.get(id).map(|(_, neighbors)| neighbors)
    }

    pub fn remove_node(&self, id: NodeId) -> (Node, Vec<NodeId>) {
        match self.nodes.remove(&id) {
            Some((node, neighbors)) => {
                for &neighbor in neighbors.iter() {
                    self.edges.remove((id, neighbor));

                    // Remove node from neighbors of the neighbor
                    let other_neighbors = self.nodes.get_mut(&neighbor).1;
                    let index = other_neighbors.iter()
                        .position(|c| c == id).unwrap();
                    other_neighbors.remove(index);
                }

                (node, neighbors)
            }
            None => panic!("invalid NodeId")
        }
    }

    pub fn add_edge(&self, a: NodeId, b: NodeId, edge: Edge) {
        assert!(!self.edges.get((a, b)).is_some());
        self.edges.set((a, b), edge);

        {
            let neighbors_a = self.nodes.get_mut(&a).1;
            assert!(neighbors_a.iter().all(|c| b != c));
            neighbors_a.push(b);
        }
        {
            let neighbors_b = self.nodes.get_mut(&a).1;
            assert!(neighbors_b.iter().all(|c| a != c));
            neighbors_b.push(a);
        }
    }

    pub fn get_edge(&self, a: NodeId, b: NodeId) -> Option<&Edge> {
        self.edges.get((a, b))
    }

    pub fn remove_edge(&self, a: NodeId, b: NodeId) -> Edge {
        assert!(self.edges.get((a, b)).is_some());

        self.edges.remove((a, b));

        {
            let neighbors_a = self.nodes.get_mut(&a).1;
            let index = neighbors_a.iter().position(|c| c == b).unwrap();
            neighbors_a.remove(index);
        }
        {
            let neighbors_b = self.nodes.get_mut(&b).1;
            let index = neighbors_b.iter().position(|c| c == b).unwrap();
            neighbors_b.remove(index);
        }
    }
}

impl Canonize for (T, T) where T: Ord {
    type Canon = (T, T);

    fn canonize(&self) -> (T, T) {
        if self.0 < self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
    }
}
