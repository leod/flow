use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::vector::Vector;

use graph::NodeIndex;
use flow::state::{self, State};

pub fn edge_velocity(
    from_idx: NodeIndex, to_idx: NodeIndex, c: &state::Edge
) -> f64 {
    if from_idx < to_idx {
        c.velocity
    } else {
        -c.velocity
    }
}

#[allow(non_snake_case)]
fn solve_pressure(state: &mut State) {
    let num_v = state.mut_idx_to_node_idx.len();
    let mut A = Matrix::<f64>::zeros(num_v, num_v); // system
    let mut b = Vector::<f64>::zeros(num_v); // rhs

    // build rows
    for (mut_idx, &node_idx) in state.mut_idx_to_node_idx.iter().enumerate() { 
        let row_id = mut_idx;

        // here the blobs have an impact, we add pressure on the rhs of each row
        //b[row_id] -= state.graph.node(node_idx).load as f64 / 10000.0;

        // step through neigbors -> either non-zero entry in matrix or add to rhs
        let neighbors = state.graph.neighbors(node_idx);
        for &(neigh_node_idx, edge_idx) in neighbors {
            let neigh_node = state.flow.node(neigh_node_idx);
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
            let edge = state.flow.edge(edge_idx);
            b[row_id] -= edge_velocity(node_idx, neigh_node_idx, edge);
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
        state.flow.node_mut(node_idx).pressure = x[mut_idx];
    }

    //println!("pressures: {:?}", (0..state.graph.num_nodes()).map(|i| state.graph.node(i).pressure).collect::<Vec<_>>());
}

fn project_velocities(state: &mut State) {
    // update velocities for all edges
    let edges = state.graph.edges().iter().enumerate();
    for (edge_idx, &(from_idx, to_idx)) in edges {
        let press_from = state.flow.node(from_idx).pressure;
        let press_to = state.flow.node(to_idx).pressure;
        
        let edge = state.flow.edge_mut(edge_idx);

        edge.old_velocity = edge.velocity;
        edge.velocity = press_from - press_to;
    }
}

fn flow(state: &mut State) {
    println!("++++++++++++++++++++++++++++++++++++");
    println!("START");
    println!("++++++++++++++++++++++++++++++++++++");
    for node_idx in 0 .. state.graph.num_nodes() {
        println!("now cell {0} has {1}, mut_idx: {2:?}", 
            node_idx, 
            state.flow.node(node_idx).load,
            state.flow.node(node_idx).mut_idx);
    }
    println!("++++++++++++++++++++++++++++++++++++");

    // backup old loads we will override with accumulation of neighbors
    for node_idx in 0 .. state.graph.num_nodes() {
        let cell = state.flow.node_mut(node_idx);
        cell.old_load = cell.load;
        cell.load = 0;
    }

    for node_idx in 0 .. state.graph.num_nodes() {
        let cell_load = state.flow.node(node_idx).old_load;

        // first get sum of outflow to get relative flow
        let mut out_flow_sum = 0.0;
        let neighbors = state.graph.neighbors(node_idx);
        for &(neigh_node_idx, edge_idx) in neighbors {
            out_flow_sum += {
                let edge = state.flow.edge(edge_idx);
                let edge_vel = edge_velocity(node_idx, neigh_node_idx, edge);
                if edge_vel > 0.0 { edge_vel } else { 0.0 }
            };
        }

        // distribute our load to neighbors respecting relative flow
        if out_flow_sum < 0.000001 {
            continue;
        }
        
        for &(neigh_node_idx, edge_idx) in state.graph.neighbors(node_idx) {
            let velocity = {
                let edge = state.flow.edge(edge_idx);
                edge_velocity(node_idx, neigh_node_idx, edge)
            };
            if velocity <= 0.0 {
                continue;
            }

            let rel_vel = velocity / out_flow_sum;

            println!("cell {0} is giving {1}: {2} % of {3}", node_idx, neigh_node_idx, rel_vel, cell_load);

            let neigh_cell = state.flow.node_mut(neigh_node_idx);
            // TODO: for now, accept that some load is lost in rounding 
            neigh_cell.load += (rel_vel * cell_load as f64).floor() as usize;
        }
    }

    // set the source/sink loads to a default value (minor TODO: maybe this should be a global setting?)
    for i in 0 .. state.source_cells.len() {
        let cell = state.flow.node_mut(state.source_cells[i]);
        cell.load = 100000;
    }
    for i in 0 .. state.sink_cells.len() {
        let cell = state.flow.node_mut(state.sink_cells[i]);
        cell.load = 0;
    }
   
    println!("++++++++++++++++++++++++++++++++++++");
    println!("END");
    println!("++++++++++++++++++++++++++++++++++++");
    for node_idx in 0 .. state.graph.num_nodes() {
        println!("now cell {0} has {1}, mut_idx: {2:?}", 
            node_idx, 
            state.flow.node(node_idx).load,
            state.flow.node(node_idx).mut_idx);

    }
    println!("++++++++++++++++++++++++++++++++++++");
}

pub fn time_step(state: &mut State, _dt: f64) {
    solve_pressure(state);
    project_velocities(state);
    flow(state);
}
