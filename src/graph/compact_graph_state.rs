use std::hash::Hash;

use super::graph::NeighborGraph;
use super::compact_graph::{NodeIndex, EdgeIndex};

// Stores state for the nodes and edges of a CompactGraph
pub struct CompactGraphState<NodeState, EdgeState> {
    // Node state, indexed by NodeIndex
    nodes: Vec<NodeState>,

    // Edge state, indexed by EdgeIndex
    edges: Vec<EdgeState>,
}

impl<NodeState, EdgeState> CompactGraphState<NodeState, EdgeState> {
    // Create a graph state for a CompactGraph. The node and edge state is
    // initialized by mapping from the underlying Graph.
    pub fn new<NodeId, Node, Edge, FNode, FEdge>(
        graph: &NeighborGraph<NodeId, Node, Edge>,
        mut f_n: FNode,
        mut f_e: FEdge,
    ) -> Self
    where
        NodeId: Copy + Eq + Ord + Hash,
        FNode: FnMut(NodeId, &Node) -> NodeState,
        FEdge: FnMut(NodeId, NodeId, &Edge) -> EdgeState,
    {
        let nodes = graph
            .nodes
            .iter()
            .map(|(&id_a, &(ref node, ref _neighbors))| f_n(id_a, node))
            .collect();

        let edges = graph
            .edges
            .iter()
            .map(|(&(id_a, id_b), edge)| f_e(id_a, id_b, edge))
            .collect();

        CompactGraphState {
            nodes: nodes,
            edges: edges,
        }
    }

    pub fn node(&self, i: NodeIndex) -> &NodeState {
        &self.nodes[i]
    }

    pub fn node_mut(&mut self, i: NodeIndex) -> &mut NodeState {
        &mut self.nodes[i]
    }

    pub fn edge(&self, i: EdgeIndex) -> &EdgeState {
        &self.edges[i]
    }

    pub fn edge_mut(&mut self, i: EdgeIndex) -> &mut EdgeState {
        &mut self.edges[i]
    }
}
