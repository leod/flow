use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

use flow::state::State;

pub fn time_step(state: &mut State, dt: f64) {
    // Without advection
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
            b[row_id] -= if node_idx < neigh_node_idx { edge.velocity } else { -edge.velocity };
            A[[row_id, row_id]] -= 1.0;
        }
    }

    // output matrix for debug
    println!("{:?}", A);

    // output rhs
    println!("{:?}", b);

    // solve this linear system (vll leo sagt 'shit')
    let x = A.solve(b).unwrap();

    // output pressures
    println!("{:?}", x);

    // update velocities

}
