use cgmath::{Zero, Vector2};

use ggez::{GameResult, Context};
use ggez::graphics;

use types::Dir;
use camera::Camera;
use circuit::{Circuit, SwitchType, Element, Component};
use flow;

pub const EDGE_LENGTH: f32 = 1.5;
pub const HALF_EDGE_LENGTH: f32 = EDGE_LENGTH / 2.0;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DrawMode {
    Real,
    Plan,
    Invalid
}

impl DrawMode {
    pub fn to_color(self) -> graphics::Color {
        match self {
            DrawMode::Real => graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            DrawMode::Plan => graphics::Color::new(0.5, 0.5, 0.5, 1.0),
            DrawMode::Invalid => graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        }
    }
}

pub struct Display {
}

impl Display {
    pub fn new() -> Display {
        Display {
        }
    }

    pub fn draw_grid_edges(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        circuit: &Circuit,
    ) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        for (&(cell_a, cell_b), &_edge) in circuit.graph().edges().iter() {
            let a = circuit.graph().get_node(cell_a).unwrap();
            let b = circuit.graph().get_node(cell_b).unwrap();

            let a_t = camera.transform(a.cast() * EDGE_LENGTH);
            let b_t = camera.transform(b.cast() * EDGE_LENGTH);

            let a_p = graphics::Point::new(a_t.x, a_t.y);
            let b_p = graphics::Point::new(b_t.x, b_t.y);

            graphics::line(ctx, &vec![a_p, b_p])?;
        }

        Ok(())
    }

    pub fn draw_component_edges(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        component: &Component
    ) -> GameResult<()> {
        for &a in component.cells.iter() {
            for &dir in &vec![Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
                let end = dir.apply(a);

                if !component.rect.is_within(end) {
                    let delta = dir.apply(Vector2::zero()).cast();
                    let b = a.cast() + delta * 0.4;

                    let a_t = camera.transform(a.cast() * EDGE_LENGTH);
                    let b_t = camera.transform(b * EDGE_LENGTH);

                    let a_p = graphics::Point::new(a_t.x, a_t.y);
                    let b_p = graphics::Point::new(b_t.x, b_t.y);

                    graphics::line(ctx, &vec![a_p, b_p])?;
                }
            }

        }

        Ok(())
    }

    pub fn draw_component(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        c: &Component,
        mode: DrawMode
    ) -> GameResult<()> {
        graphics::set_color(ctx, mode.to_color())?;

        let p_t = camera.transform(c.pos.cast() * EDGE_LENGTH);

        match c.element {
            Element::Node => {
                let r = graphics::Rect {
                    x: p_t.x,
                    y: p_t.y,
                    w: camera.transform_distance(EDGE_LENGTH / 2.0),
                    h: camera.transform_distance(EDGE_LENGTH / 2.0)
                };

                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
            }
            Element::Switch(kind) => {
                let control_p_t =
                    camera.transform(c.cells[0].cast() * EDGE_LENGTH);
                let flow_p_t = camera.transform(c.cells[1].cast() * EDGE_LENGTH);
                
                let control_draw_mode = match kind {
                    SwitchType::On => graphics::DrawMode::Fill,
                    SwitchType::Off => graphics::DrawMode::Line
                };

                graphics::circle(ctx,
                    control_draw_mode,
                    graphics::Point {
                        x: control_p_t.x,
                        y: control_p_t.y
                    },
                    camera.transform_distance(HALF_EDGE_LENGTH * 0.75),
                    50)?;
                
                let r = graphics::Rect {
                    x: flow_p_t.x,
                    y: flow_p_t.y,
                    w: camera.transform_distance(EDGE_LENGTH),
                    h: camera.transform_distance(EDGE_LENGTH)
                };
                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
            }
            Element::Source | Element::Sink => {
                let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                    * EDGE_LENGTH;
                let shift = c.size().cast() * HALF_EDGE_LENGTH;
                let trans_size = camera.transform_delta(size);
                let trans_shift = camera.transform_delta(shift);
                let center = p_t + trans_shift;

                self.draw_component_edges(ctx, camera, c)?;

                let r = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size.x,
                    h: trans_size.y
                };
                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;

                graphics::circle(ctx, graphics::DrawMode::Fill,
                                 graphics::Point { x: center.x, y: center.y },
                                 camera.transform_distance(size.x / 2.0),
                                 50)?;

                graphics::set_color(ctx,
                                    graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
                graphics::circle(ctx, graphics::DrawMode::Fill,
                                 graphics::Point { x: center.x, y: center.y },
                                 camera.transform_distance(size.x / 2.0 - 0.05),
                                 50)?;
            }
            _ => {}
        }
        
        Ok(())
    }


    pub fn draw_components(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        circuit: &Circuit,
    ) -> GameResult<()> {
        for (ref _id, ref c) in circuit.components().iter() {
            self.draw_component(ctx, camera, c, DrawMode::Real)?;
        }

        Ok(())
    }

    pub fn draw_flow(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        circuit: &Circuit,
        state: &flow::State
    ) -> GameResult<()> {
        for (&id, ref c) in circuit.components().iter() {
            for (cell_index, _pos) in c.cells.iter().enumerate() {
                let cell_id = (id, cell_index);
                let node_index = state.graph.node_index(cell_id);
                let cell = state.flow.node(node_index);

                let p = c.cells[cell_index].cast();
                let p_t = camera.transform(p * EDGE_LENGTH);
                let size = camera.transform_distance(EDGE_LENGTH * 0.4);
                let r = graphics::Rect {
                    x: p_t.x,
                    y: p_t.y,
                    w: size,
                    h: size
                };

                let pressure = cell.pressure as f32;
                graphics::set_color(ctx,
                    graphics::Color::new(1.0 * (pressure/100.0),
                                         0.0,
                                         1.0 * (1.0 - pressure/100.0),
                                         1.0))?;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;
            }
        }
        
        graphics::set_color(ctx, graphics::Color::new(1.0, 0.4, 0.0, 1.0))?;
        
        let max_size = EDGE_LENGTH / 4.0;
        
        for (&(cell_id_a, cell_id_b), &_edge) in circuit.graph().edges().iter() {
            let node_index_a = state.graph.node_index(cell_id_a);
            let node_index_b = state.graph.node_index(cell_id_b);
            
            let a_p = *circuit.graph().get_node(cell_id_a).unwrap();
            let b_p = *circuit.graph().get_node(cell_id_b).unwrap();
            
            let edge_index = state.graph.edge_index(cell_id_a, cell_id_b);
            let edge = state.flow.edge(edge_index);
            
            let percent = edge.flow.abs() as f32 / 100000.0; // TODO
            let size = max_size * percent;
            
            // Order from/to according to node indices
            let (from_p, to_p) =
                if node_index_a < node_index_b {
                    (a_p, b_p)
                } else {
                    (b_p, a_p)
                };
            
            // Flip arrow if flow is negative
            let (from_p, to_p) =
                if edge.flow > 0 {
                    (from_p, to_p)
                } else {
                    (to_p, from_p)
                };
            
            let dir = Dir::from_coords(from_p, to_p);
            let orth_dir = dir.rotate_cw();
            
            // Corner position of the nodes
            let start_p = from_p.cast() + dir.delta().cast() / 4.0;
            let end_p = to_p.cast() + dir.invert().delta().cast() / 4.0;
            
            let x = start_p + orth_dir.delta().cast() * size;
            let y = start_p + orth_dir.invert().delta().cast() * size;
            let z = end_p;
            
            let x_t = camera.transform(x * EDGE_LENGTH);
            let y_t = camera.transform(y * EDGE_LENGTH);
            let z_t = camera.transform(z * EDGE_LENGTH);
            
            let vertices = vec![
                graphics::Point {
                    x: x_t.x, y: x_t.y
                },
                graphics::Point {
                    x: y_t.x, y: y_t.y
                },
                graphics::Point {
                    x: z_t.x, y: z_t.y
                }
            ];
            
            graphics::polygon(ctx, graphics::DrawMode::Fill, &vertices)?;
        }
        
        Ok(())
    }
}

