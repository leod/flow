use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::vector::Vector;

use flow::state::State;

#[allow(non_snake_case)]
pub fn time_step(state: &mut State, _dt: f64) {
    // solve for pressure
    let num_v = state.mut_idx_to_node_idx.len();
    let mut A = Matrix::<f64>::zeros(num_v, num_v); // system
    let mut b = Vector::<f64>::zeros(num_v); // rhs

    // build rows
    for (mut_idx, &node_idx) in state.mut_idx_to_node_idx.iter().enumerate() { 
        let row_id = mut_idx;

        // step through neigbors -> either non-zero entry in matrix or add to rhs
        for &(neigh_node_idx, edge_idx) in state.graph.neighbors(node_idx).iter() {
            let neigh_node = state.graph.node(neigh_node_idx);
            if let Some(neigh_mut_idx) = neigh_node.mut_idx {
                // mutable neighbor -> need to compute pressure
                let col_id = neigh_mut_idx;

                A[[row_id, col_id]] = 1.0;
            } else {
                // immutable neighbor -> need to add to right side
                b[row_id] -= neigh_node.pressure; 
            }

            // substract flow on rhs
            // TODO: we have to take care in which direction the flow goes
            let edge = state.graph.edge(edge_idx);
            b[row_id] -=
                if node_idx < neigh_node_idx {
                    edge.velocity
                } else {
                    -edge.velocity
                };
            A[[row_id, row_id]] -= 1.0;
        }
    }

    // output matrix for debug
    //println!("A: {:?}", A);

    // output rhs
    //println!("b: {:?}", b);

    // solve this linear system (vll leo sagt 'shit')
    //let x = A.solve(b).unwrap();
    
    let L = (-A).cholesky().unwrap();
    //println!("L: {:?}", L);
    let y = L.solve_l_triangular((-b)).unwrap();
    let x = L.transpose().solve_u_triangular(y).unwrap();

    // output pressures
    //println!("{:?}", x);

    // write pressures
    for (mut_idx, &node_idx) in state.mut_idx_to_node_idx.iter().enumerate() { 
        state.graph.node_mut(node_idx).pressure = x[mut_idx];
    }

    //println!("pressures: {:?}", (0..state.graph.num_nodes()).map(|i| state.graph.node(i).pressure).collect::<Vec<_>>());

    // update velocities for all edges
    for node_idx in 0 .. state.graph.num_nodes() {
        // take care that we only update each edge once
        for &(neigh_node_idx, edge_idx) in state.graph.neighbors(node_idx).clone().iter() {
            if node_idx < neigh_node_idx {
                let press_i = state.graph.node(node_idx).pressure;
                let press_j = state.graph.node(neigh_node_idx).pressure;

                let edge = state.graph.edge_mut(edge_idx);

                edge.old_velocity = edge.velocity;
                edge.velocity += press_j - press_i;
            }
        }
    }
}

