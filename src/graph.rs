use std::collections::HashMap;
use std::hash::Hash;

use canon_map::{Canonize, CanonMap};

pub struct Graph<NodeId: Copy + Eq + Ord + Hash, Node, Edge> {
    nodes: HashMap<NodeId, Node>,
    edges: CanonMap<(NodeId, NodeId), Edge>
}

pub type NeighboredGraph<NodeId, Node, Edge> =
    Graph<NodeId, (Node, Vec<NodeId>), Edge>;

type NodeIndex = usize;
type EdgeIndex = usize;

pub struct GraphState<NodeId: Copy + Eq + Ord + Hash, Node, Edge> {
    // Map from the graph to indices in our state vectors
    indices: Graph<NodeId, NodeIndex, EdgeIndex>,

    nodes: Vec<(Node, Vec<(NodeIndex, EdgeIndex)>)>,
    edges: Vec<Edge>,
}

impl<NodeId: Copy + Eq + Ord + Hash + Copy, Node, Edge>
    NeighboredGraph<NodeId, Node, Edge> {
    pub fn new() -> Self {
        Graph {
            edges: CanonMap::new(),
            nodes: HashMap::new()
        }
    }

    pub fn add_node(&mut self, id: NodeId, node: Node) {
        assert!(!self.nodes.get(&id).is_some());

        self.nodes.insert(id, (node, Vec::new()));
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id).map(|&(ref node, _)| node)
    }

    pub fn get_neighbors(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.nodes.get(&id).map(|&(_, ref neighbors)| neighbors)
    }

    pub fn remove_node(&mut self, id: NodeId) -> (Node, Vec<NodeId>) {
        match self.nodes.remove(&id) {
            Some((node, neighbors)) => {
                for &neighbor in neighbors.iter() {
                    self.edges.remove((id, neighbor));

                    // Remove node from neighbors of the neighbor
                    let ref mut other_neighbors =
                        self.nodes.get_mut(&neighbor).unwrap().1;
                    let index = other_neighbors.iter()
                        .position(|&c| c == id).unwrap();
                    other_neighbors.remove(index);
                }

                (node, neighbors)
            }
            None => panic!("invalid NodeId")
        }
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId, edge: Edge) {
        assert!(!self.edges.get((a, b)).is_some());
        self.edges.set((a, b), edge);

        {
            let ref mut neighbors_a = self.nodes.get_mut(&a).unwrap().1;
            assert!(neighbors_a.iter().all(|&c| b != c));
            neighbors_a.push(b);
        }
        {
            let ref mut neighbors_b = self.nodes.get_mut(&b).unwrap().1;
            assert!(neighbors_b.iter().all(|&c| a != c));
            neighbors_b.push(a);
        }
    }

    pub fn get_edge(&self, a: NodeId, b: NodeId) -> Option<&Edge> {
        self.edges.get((a, b))
    }

    pub fn remove_edge(&mut self, a: NodeId, b: NodeId) -> Edge {
        {
            let ref mut neighbors_a = self.nodes.get_mut(&a).unwrap().1;
            let index = neighbors_a.iter().position(|&c| c == b).unwrap();
            neighbors_a.remove(index);
        }
        {
            let ref mut neighbors_b = self.nodes.get_mut(&b).unwrap().1;
            let index = neighbors_b.iter().position(|&c| c == a).unwrap();
            neighbors_b.remove(index);
        }

        assert!(self.edges.get((a, b)).is_some());

        self.edges.remove((a, b)).unwrap()
    }

    pub fn edges(&self) -> &CanonMap<(NodeId, NodeId), Edge> {
        &self.edges
    }
}

impl<NodeId: Copy + Eq + Ord + Hash, Node, Edge> GraphState<NodeId, Node, Edge> {
    pub fn new<N, E, F_N, F_E>(
        graph: &NeighboredGraph<NodeId, N, E>,
        f_n: F_N,
        f_e: F_E
    ) -> Self
    where F_N: Fn(NodeId, &N) -> Node, F_E: Fn(NodeId, NodeId, &E) -> Edge {
        let node_indices = graph.nodes.iter()
            .enumerate()
            .map(|(i, (&id, _))| (id, i))
            .collect::<HashMap<NodeId, NodeIndex>>();
        let edge_indices = graph.edges.iter()
            .enumerate()
            .map(|(i, (&(id_a, id_b), _))| ((id_a, id_b), i))
            .collect::<CanonMap<(NodeId, NodeId), EdgeIndex>>();  

        let nodes = graph.nodes.iter().map(|(&id_a, &(ref node, ref neighbors))| {
                let neighbor_indices = neighbors.iter()
                    .map(|id_b| *node_indices.get(id_b).unwrap());
                let edge_indices = neighbors.iter()
                    .map(|&id_b| *edge_indices.get((id_a, id_b)).unwrap());
                let neighbors = neighbor_indices.zip(edge_indices).collect();
                (f_n(id_a, node), neighbors)
            }).collect();
        
        let edges = graph.edges.iter().map(|(&(id_a, id_b), edge)|
                f_e(id_a, id_b, edge) 
            ).collect();

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

    pub fn node(&self, i: NodeIndex) -> &Node {
        &self.nodes[i].0
    }

    pub fn node_mut(&mut self, i: NodeIndex) -> &mut Node {
        &mut self.nodes[i].0
    }

    pub fn neighbors(&self, i: NodeIndex) -> &Vec<(NodeIndex, EdgeIndex)> {
        &self.nodes[i].1
    }

    pub fn edge(&self, i: EdgeIndex) -> &Edge {
        &self.edges[i]
    }

    pub fn edge_mut(&mut self, i: EdgeIndex) -> &mut Edge {
        &mut self.edges[i]
    }
}

impl<NodeId, Node, Edge> Clone for Graph<NodeId, Node, Edge>
where NodeId: Clone + Copy + Eq + Ord + Hash, Node: Clone, Edge: Clone {
    fn clone(&self) -> Self {
        Graph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone()
        }
    }
}

impl<T> Canonize for (T, T) where T: Copy + Ord + Eq + Hash {
    type Canon = (T, T);

    fn canonize(&self) -> (T, T) {
        if self.0 < self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
    }
}

