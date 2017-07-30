use rulinalg;
use rulinalg::matrix::{Matrix, BaseMatrix};
use rulinalg::vector::Vector;

use circuit::{Element, SwitchType};
use flow::state::{State, edge_quantity};

#[allow(non_snake_case)]
fn solve_pressure(state: &mut State) -> Result<(), rulinalg::error::Error> {
    let num_v = state.mut_idx_to_node_idx.len();

    if num_v == 0 {
        return Ok(());
    }

    let mut A = Matrix::<f64>::zeros(num_v, num_v); // system
    let mut b = Vector::<f64>::zeros(num_v); // rhs

    for node_idx in 0..state.graph.num_nodes() {
        if !state.flow.node(node_idx).bound_pressure {
            state.flow.node_mut(node_idx).pressure = 0.0;
        }
    }

    // build rows
    for (mut_idx, &node_idx) in state.mut_idx_to_node_idx.iter().enumerate() {
        let row_id = mut_idx;

        // here the blobs have an impact, we add pressure on the rhs of each row
        //b[row_id] -= state.flow.node(node_idx).load as f64;

        // step through neigbors -> either non-zero entry in matrix or add to rhs
        let neighbors = state.graph.neighbors(node_idx);
        for &(neigh_node_idx, edge_idx) in neighbors {
            let edge = state.flow.edge(edge_idx);
            if !edge.enabled {
                continue;
            }

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
            //b[row_id] -= edge_quantity(node_idx, neigh_node_idx, edge.velocity);
            A[[row_id, row_id]] -= 1.0;
        }
    }

    // output matrix for debug
    //println!("A: {:?}", A);

    // output rhs
    //println!("b: {:?}", b);

    // solve this linear system (vll leo sagt 'shit')
    //let x = A.solve(b).unwrap();

    let L = (-A).cholesky()?;
    //println!("L: {:?}", L);
    let y = L.solve_l_triangular((-b))?;
    let x = L.transpose().solve_u_triangular(y)?;

    // output pressures
    //println!("{:?}", x);

    // write pressures
    for (mut_idx, &node_idx) in state.mut_idx_to_node_idx.iter().enumerate() {
        state.flow.node_mut(node_idx).pressure = x[mut_idx];
    }

    //println!("pressures: {:?}", (0..state.graph.num_nodes()).map(|i| state.flow.node(i).pressure).collect::<Vec<_>>());

    Ok(())
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

fn update_components(state: &mut State) {
    for ref component in state.components.iter() {
        match component.element {
            Element::Switch(kind) => {
                let threshold = 0.01;
                let enabled = {
                    let control_node_idx = component.cells[0];
                    let control_cell = state.flow.node(control_node_idx);

                    match kind {
                        SwitchType::On => control_cell.in_flow > threshold,
                        SwitchType::Off => control_cell.in_flow < threshold,
                    }
                };

                let flow_node_idx = component.cells[1];
                for &(_, edge_idx) in state.graph.neighbors(flow_node_idx) {
                    let edge = state.flow.edge_mut(edge_idx);
                    edge.enabled = enabled;
                }
            }
            Element::Power => {
                let threshold = 0.01;
                let enabled = {
                    let control_node_idx = component.cells[0];
                    let control_cell = state.flow.node(control_node_idx);
                    control_cell.in_flow > threshold
                };

                let power_cell_idx = component.cells[1];
                let power_cell = state.flow.node_mut(power_cell_idx);
                power_cell.bound_pressure = enabled;
                power_cell.pressure = 100.0;
            }
            _ => {}
        }
    }
}

fn flow(state: &mut State) {
    /*println!("++++++++++++++++++++++++++++++++++++");
    println!("START");
    println!("++++++++++++++++++++++++++++++++++++");
    for node_idx in 0 .. state.graph.num_nodes() {
        println!("now cell {0} has {1}, mut_idx: {2:?}", 
            node_idx, 
            state.flow.node(node_idx).load,
            state.flow.node(node_idx).mut_idx);
    }
    println!("++++++++++++++++++++++++++++++++++++");*/

    // set the source/sink loads to a default value (minor TODO: maybe this should be a global setting?)
    for i in 0..state.source_cells.len() {
        let cell = state.flow.node_mut(state.source_cells[i]);
        cell.load = if cell.enabled { 100000.0 } else { 0.0 };
    }
    for i in 0..state.sink_cells.len() {
        let cell = state.flow.node_mut(state.sink_cells[i]);
        cell.load = 0.0;
    }

    // backup old loads we will override with accumulation of neighbors
    for node_idx in 0..state.graph.num_nodes() {
        let cell = state.flow.node_mut(node_idx);
        cell.old_load = cell.load;
        //cell.load = 0;
        cell.in_flow = 0.0;
        cell.out_flow = 0.0;
    }

    for edge_idx in 0..state.graph.num_edges() {
        state.flow.edge_mut(edge_idx).flow = 0.0;
    }

    for node_idx in 0..state.graph.num_nodes() {
        let cell_load = state.flow.node(node_idx).old_load;

        // first get sum of outflow to get relative flow
        let mut out_flow_sum = 0.0;
        let neighbors = state.graph.neighbors(node_idx);
        for &(neigh_node_idx, edge_idx) in neighbors {
            out_flow_sum += {
                let edge = state.flow.edge(edge_idx);
                let edge_vel = edge_quantity(node_idx, neigh_node_idx, edge.velocity);
                if edge.enabled && edge_vel > 0.0 {
                    edge_vel
                } else {
                    0.0
                }
            };
        }

        // distribute our load to neighbors respecting relative flow
        for &(neigh_node_idx, edge_idx) in state.graph.neighbors(node_idx) {
            let velocity = {
                let edge = state.flow.edge(edge_idx);
                if edge.enabled {
                    edge_quantity(node_idx, neigh_node_idx, edge.velocity)
                } else {
                    0.0
                }
            };
            if velocity <= 0.0 {
                continue;
            }

            let rel_vel = velocity / out_flow_sum;

            // TODO: for now, accept that some load is lost in rounding
            let flow = (rel_vel * cell_load).min(velocity);

            /*println!("cell {0} is giving {1}: {2}% of {3}: {4}",
                node_idx, neigh_node_idx, rel_vel * 100.0, cell_load, flow);*/

            {
                let neigh_node = state.flow.node_mut(neigh_node_idx);
                neigh_node.load += flow;
                neigh_node.in_flow += flow;
            }
            {
                let node = state.flow.node_mut(node_idx);
                node.load -= flow;
                node.out_flow += flow;
            }

            state.flow.edge_mut(edge_idx).flow += edge_quantity(node_idx, neigh_node_idx, flow);
        }
    }

    /*println!("++++++++++++++++++++++++++++++++++++");
    println!("END");
    println!("++++++++++++++++++++++++++++++++++++");
    for node_idx in 0 .. state.graph.num_nodes() {
        println!("now cell {0} has {1}, mut_idx: {2:?}", 
            node_idx, 
            state.flow.node(node_idx).load,
            state.flow.node(node_idx).mut_idx);

    }
    println!("++++++++++++++++++++++++++++++++++++");*/
}

pub fn time_step(state: &mut State, _dt: f64) {
    update_components(state);
    state.update_mut_indices();
    if let Err(err) = solve_pressure(state) {
        println!("Can't solve pressure: {:?}", err);
        return;
    }
    project_velocities(state);
    flow(state);
}
