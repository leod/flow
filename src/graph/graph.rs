use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use canon_map::{Canonize, CanonMap};

// A dynamic undirected graph, with some annotation for each node and edge.
// Nodes are identified by a NodeId, and edges by a pair of NodeIds. This
// struct is designed for reasonably fast lookup and mutation.
pub struct Graph<NodeId: Copy + Eq + Ord + Hash, Node, Edge> {
    pub nodes: HashMap<NodeId, Node>,
    pub edges: CanonMap<(NodeId, NodeId), Edge>,
}

// A graph in which each node additionally stores the NodeIds of its neighbors
pub type NeighborGraph<NodeId, Node, Edge> = Graph<
    NodeId,
    (Node, Vec<NodeId>),
    Edge,
>;

// Implement the redundant information in a NeighborGraph, so that the neighbor
// lists are always up to date
impl<NodeId, Node, Edge> NeighborGraph<NodeId, Node, Edge>
where
    NodeId: Copy + Eq + Ord + Hash,
    Node: Clone,
    Edge: Clone,
{
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: CanonMap::new(),
        }
    }

    pub fn nodes(&self) -> &HashMap<NodeId, (Node, Vec<NodeId>)> {
        &self.nodes
    }

    pub fn edges(&self) -> &CanonMap<(NodeId, NodeId), Edge> {
        &self.edges
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
                    let ref mut other_neighbors = self.nodes.get_mut(&neighbor).unwrap().1;
                    let index = other_neighbors.iter().position(|&c| c == id).unwrap();
                    other_neighbors.remove(index);
                }

                (node, neighbors)
            }
            None => panic!("invalid NodeId"),
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

    pub fn subgraph(&self, node_ids: &HashSet<NodeId>) -> NeighborGraph<NodeId, Node, Edge> {
        let nodes = self.nodes
            .iter()
            .filter(|&(id, _node)| node_ids.contains(id))
            .map(|(&id, &(ref node, ref neighbors))| {
                let neighbors = neighbors
                    .iter()
                    .filter(|&n_id| node_ids.contains(n_id))
                    .map(|&n_id| n_id)
                    .collect();
                (id, (node.clone(), neighbors))
            })
            .collect();
        let edges = self.edges
            .iter()
            .filter(|&(&(ref a_id, ref b_id), _edge)| {
                node_ids.contains(a_id) && node_ids.contains(b_id)
            })
            .map(|(&nodes, edge)| (nodes, edge.clone()))
            .collect();
        NeighborGraph { nodes, edges }
    }
}

impl<NodeId, Node, Edge> Clone for Graph<NodeId, Node, Edge>
where
    NodeId: Clone
        + Copy
        + Eq
        + Ord
        + Hash,
    Node: Clone,
    Edge: Clone,
{
    fn clone(&self) -> Self {
        Graph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }
}

impl<T> Canonize for (T, T)
where
    T: Copy + Ord + Eq + Hash,
{
    type Canon = (T, T);

    fn canonize(&self) -> (T, T) {
        if self.0 < self.1 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        }
    }
}
