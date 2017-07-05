use std::collections::HashMap;
use std::hash::Hash;

use canon_map::{Canonize, CanonMap};

pub struct Graph<NodeId: Eq + Ord + Hash, Node, Edge> {
    nodes: HashMap<NodeId, (Node, Vec<NodeId>)>,
    edges: CanonMap<(NodeId, NodeId), Edge>
}

type NeighboredGraph<NodeId, Node, Edge> =
    Graph<NodeId, (Node, Vec<NodeId>), Edge>;

type NodeIndex = usize;
type EdgeIndex = usize;

pub struct GraphState<NodeId: Eq + Ord + Hash, Node, Edge> {
    // Map from the graph to indices in our state vectors
    indices: Graph<NodeId, NodeIndex, EdgeIndex>,

    nodes: Vec<(Node, Vec<EdgeIndex>)>,
    edges: Vec<Edge>
}

impl<NodeId: Eq + Ord + Hash + Copy, Node, Edge>
    NeighboredGraph<NodeId, Node, Edge> {
    pub fn new() -> Self {
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

impl<NodeId: Eq + Ord + Hash, Node, Edge> GraphState<NodeId, Node, Edge> {
    pub fn new<N, E, F_N, F_E>(
        graph: &NeighboredGraph<NodeId, N, E>,
        f_n: F_N,
        f_e: F_E
    ) -> Self
    where F_N: FnMut(&N) -> Node, F_E: FnMut(&E) -> Edge {
        let node_indices = graph.nodes.iter()
            .enumerate()
            .map(|(i, &(id, _))| (id, i))
            .collect::<HashMap<NodeId, NodeIndex>>();
        let edge_indices = graph.edges.iter()
            .enumerate()
            .map(|(i, &((id_a, id_b), _))| ((id_a, id_b), i))
            .collect::<HashMap<(NodeId, NodeId), EdgeIndex>>();  

        let nodes = graph.nodes.map(|(id, &(node, neighbors))| {
                let neighbor_indices = neighbors.iter()
                    .map(|id| node_indices.get(id).unwrap())
                    .collect();
                (f_n(node), neighbor_indices)
            }).collect();
        
        let edges = graph.edges.map(|&((id_a, id_b), edge)|
                f_e(edge) 
            );

        let indices = Graph {
            nodes: node_indices,
            edges: edge_indices
        };

        GraphState {
            indices: indices,
            nodes: nodes,
            edges: edges
        }
    }
}

impl<T> Canonize for (T, T) where T: Ord {
    type Canon = (T, T);

    fn canonize(&self) -> (T, T) {
        if self.0 < self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
    }
}
