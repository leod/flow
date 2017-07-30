use std::collections::HashMap;
use std::hash::Hash;

use canon_map::CanonMap;

use super::graph::{Graph, NeighborGraph};

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

// A static undirected graph that has the same structure as a Graph. Stores the
// graph structure in Vecs for efficient lookup. Maps from the Graph's NodeIds
// and NodeId pairs (which identify edges) to NodeIndex and EdgeIndex, which can
// be used for lookups in the Vecs. Note that CompactGraph is only about the
// structure of the graph and does not store any additional information for the
// nodes and edges.
pub struct CompactGraph<NodeId: Copy + Ord + Hash> {
    // Map from NodeIds to NodeIndex
    indices: Graph<NodeId, NodeIndex, EdgeIndex>,

    // List of neighbors (and the corresponding edge) for each node. Nodes are
    // indexed by NodeIndex.
    neighbors: Vec<Vec<(NodeIndex, EdgeIndex)>>,

    // Indices of the two nodes of an edge, such that the first index is
    // smaller than the second index. Edges are indexed by EdgeIndex.
    edges: Vec<(NodeIndex, NodeIndex)>,
}

impl<NodeId> CompactGraph<NodeId>
where
    NodeId: Copy + Eq + Ord + Hash,
{
    // Create a compact graph corresponding to a Graph. This makes it possible
    // to store state for each node and edge of a Graph with efficient lookup
    // and storage by using a CompactGraphState.
    pub fn new<Node, Edge>(graph: &NeighborGraph<NodeId, Node, Edge>) -> Self {
        let node_indices = graph
            .nodes
            .iter()
            .enumerate()
            .map(|(i, (&id, _))| (id, i))
            .collect::<HashMap<NodeId, NodeIndex>>();
        let edge_indices =
            graph
                .edges
                .iter()
                .enumerate()
                .map(|(i, (&(id_a, id_b), _))| ((id_a, id_b), i))
                .collect::<CanonMap<(NodeId, NodeId), EdgeIndex>>();

        let neighbors = graph
            .nodes
            .iter()
            .map(|(&id_a, &(ref _node, ref neighbors))| {
                let neighbor_indices = neighbors.iter().map(|id_b| {
                    *node_indices.get(id_b).unwrap()
                });
                let edge_indices = neighbors.iter().map(|&id_b| {
                    *edge_indices.get((id_a, id_b)).unwrap()
                });
                neighbor_indices.zip(edge_indices).collect()
            })
            .collect();

        let edges = graph
            .edges
            .iter()
            .map(|(&(id_a, id_b), _edge)| {
                let index_a = *node_indices.get(&id_a).unwrap();
                let index_b = *node_indices.get(&id_b).unwrap();

                // Ensure that we use the smaller NodeIndex for the first element
                if index_a < index_b {
                    (index_a, index_b)
                } else {
                    assert!(index_a != index_b);
                    (index_b, index_a)
                }
            })
            .collect();

        let indices = Graph {
            nodes: node_indices,
            edges: edge_indices,
        };

        CompactGraph {
            indices: indices,
            neighbors: neighbors,
            edges: edges,
        }
    }

    pub fn edges(&self) -> &[(NodeIndex, NodeIndex)] {
        &self.edges
    }

    pub fn node_index(&self, id: NodeId) -> NodeIndex {
        *self.indices.nodes.get(&id).unwrap()
    }

    pub fn neighbors(&self, i: NodeIndex) -> &[(NodeIndex, EdgeIndex)] {
        &self.neighbors[i]
    }

    pub fn num_nodes(&self) -> usize {
        self.neighbors.len()
    }

    pub fn edge(&self, i: EdgeIndex) -> (NodeIndex, NodeIndex) {
        self.edges[i]
    }

    pub fn edge_index(&self, id_a: NodeId, id_b: NodeId) -> EdgeIndex {
        *self.indices.edges.get((id_a, id_b)).unwrap()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}
