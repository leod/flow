use cgmath::{Zero, Vector2};

use ggez::{GameResult, Context};
use ggez::graphics::{self, Drawable};

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

// TODO: Clean up: significant overlap between draw_component and draw_flow
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
            let a = *circuit.graph().get_node(cell_a).unwrap();
            let b = *circuit.graph().get_node(cell_b).unwrap();
            let dir = Dir::from_coords(a, b);
            
            let a_corner = a.cast() + dir.delta().cast() * 0.25;
            let b_corner = b.cast() + dir.invert().delta().cast() * 0.25;

            let a_t = camera.transform(a_corner.cast() * EDGE_LENGTH);
            let b_t = camera.transform(b_corner.cast() * EDGE_LENGTH);

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
        for (cell_index, &a_coords) in component.cells.iter().enumerate() {
            for &dir in component.cell_edges[cell_index].iter() {
                let end = dir.apply(a_coords);

                if !component.rect.is_within(end) {
                    let delta = dir.apply(Vector2::zero()).cast();
                    let a = a_coords.cast() + delta * 0.25;
                    let b = a + delta * 0.2;

                    let a_t = camera.transform(a * EDGE_LENGTH);
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
        font: &graphics::Font,
        camera: &Camera,
        c: &Component,
        mode: DrawMode
    ) -> GameResult<()> {
        graphics::set_color(ctx, mode.to_color())?;

        let p_t = camera.transform(c.pos.cast() * EDGE_LENGTH);

        match &c.element {
            &Element::Node => {
                let r = graphics::Rect {
                    x: p_t.x,
                    y: p_t.y,
                    w: camera.transform_distance(EDGE_LENGTH / 2.0),
                    h: camera.transform_distance(EDGE_LENGTH / 2.0)
                };

                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
            }
            &Element::Bridge => {
                let r = graphics::Rect {
                    x: p_t.x,
                    y: p_t.y,
                    w: camera.transform_distance(EDGE_LENGTH / 2.0),
                    h: camera.transform_distance(EDGE_LENGTH / 2.0)
                };
                let inner_r = graphics::Rect {
                    x: p_t.x,
                    y: p_t.y,
                    w: camera.transform_distance(EDGE_LENGTH / 4.0),
                    h: camera.transform_distance(EDGE_LENGTH / 4.0)              
                };

                graphics::set_color(ctx, mode.to_color())?;
                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
                graphics::rectangle(ctx, graphics::DrawMode::Line, inner_r)?;
                
                let left = Dir::Left.rotate_cw_n(c.rotation_cw);
                let a = c.pos.cast() + left.delta().cast() * 0.25;
                let b = a + left.invert().delta().cast() * 0.5;
                
                let a_end = a + left.invert().delta().cast() * 0.125;
                let b_end = b + left.delta().cast() * 0.125;
                
                let a_t = camera.transform(a * EDGE_LENGTH);
                let b_t = camera.transform(b * EDGE_LENGTH);
                let a_end_t = camera.transform(a_end * EDGE_LENGTH);
                let b_end_t = camera.transform(b_end * EDGE_LENGTH);
                
                let a_p = graphics::Point::new(a_t.x, a_t.y);
                let b_p = graphics::Point::new(b_t.x, b_t.y);
                let a_end_p = graphics::Point::new(a_end_t.x, a_end_t.y);
                let b_end_p = graphics::Point::new(b_end_t.x, b_end_t.y);
                
                graphics::line(ctx, &vec![a_p, a_end_p])?;
                graphics::line(ctx, &vec![b_p, b_end_p])?;
            }
            &Element::Switch(kind) => {
                let left_dir = Dir::Left.rotate_cw_n(c.rotation_cw);
                let flow_p = c.cells[1].cast();
                let control_p = flow_p + left_dir.delta().cast() * 0.25;

                let flow_p_t = camera.transform(flow_p * EDGE_LENGTH);
                let control_p_t = camera.transform(control_p * EDGE_LENGTH);

                graphics::circle(ctx,
                    graphics::DrawMode::Fill,
                    graphics::Point {
                        x: control_p_t.x,
                        y: control_p_t.y
                    },
                    camera.transform_distance(HALF_EDGE_LENGTH * 0.3),
                    50)?;

                let r = graphics::Rect {
                    x: flow_p_t.x,
                    y: flow_p_t.y,
                    w: camera.transform_distance(HALF_EDGE_LENGTH),
                    h: camera.transform_distance(HALF_EDGE_LENGTH)
                };
                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
                            
                if kind == SwitchType::Off {
                    graphics::set_color(ctx,
                        graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
                    graphics::circle(ctx,
                        graphics::DrawMode::Fill,
                        graphics::Point {
                            x: control_p_t.x,
                            y: control_p_t.y
                        },
                        camera.transform_distance(HALF_EDGE_LENGTH * 0.25),
                        50)?;
                }
            }
            &Element::Source | &Element::Sink => {
                let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                    * EDGE_LENGTH;
                let trans_size = camera.transform_delta(size);
                let shift = c.size().cast() * HALF_EDGE_LENGTH;
                let trans_shift = camera.transform_delta(shift);
                let center = p_t + trans_shift;

                //self.draw_component_edges(ctx, camera, c)?;

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

                if c.element == Element::Sink {
                    graphics::set_color(ctx,
                                        graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
                    graphics::circle(ctx, graphics::DrawMode::Fill,
                                     graphics::Point { x: center.x, y: center.y },
                                     camera.transform_distance(size.x / 2.0 - 0.05),
                                     50)?;
                }
            }
            &Element::Input { size: _ } => {
                let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                    * EDGE_LENGTH;
                let size_small = (c.size().cast() + Vector2::new(0.25, 0.25))
                    * EDGE_LENGTH;
                let shift = c.size().cast() * HALF_EDGE_LENGTH;
                let trans_size = camera.transform_delta(size);
                let trans_size_small = camera.transform_delta(size_small);
                let trans_shift = camera.transform_delta(shift);
                let center = p_t + trans_shift;

                let r = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size.x,
                    h: trans_size.y
                };
                graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;
                
                let r_small = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size_small.x,
                    h: trans_size_small.y
                };
                graphics::set_color(ctx,
                    graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
                graphics::rectangle(ctx, graphics::DrawMode::Fill, r_small)?;
            }
            &Element::Output { size: _ } => {
                let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                    * EDGE_LENGTH;
                let size_small = (c.size().cast() + Vector2::new(0.25, 0.25))
                    * EDGE_LENGTH;
                let shift = c.size().cast() * HALF_EDGE_LENGTH;
                let trans_size = camera.transform_delta(size);
                let trans_size_small = camera.transform_delta(size_small);
                let trans_shift = camera.transform_delta(shift);
                let center = p_t + trans_shift;

                let r = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size.x,
                    h: trans_size.y
                };
                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
                
                let r_small = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size_small.x,
                    h: trans_size_small.y
                };
                graphics::rectangle(ctx, graphics::DrawMode::Line, r_small)?;
            }
            &Element::Power => {
                // Corner position of the nodes
                let dir = Dir::Left.rotate_cw_n(c.rotation_cw);
                let orth_dir = dir.rotate_cw();
                let left = c.pos.cast() + dir.delta().cast() / 4.0;
                let x = left + orth_dir.delta().cast() / 4.0;
                let y = left + orth_dir.invert().delta().cast() / 4.0;
                let z = c.pos.cast() + dir.invert().delta().cast() / 4.0;
                
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
            &Element::Chip(ref chip_id, ref _chip_descr) => {
                self.draw_component_edges(ctx, camera, c)?;

                let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                    * EDGE_LENGTH;
                let trans_size = camera.transform_delta(size);
                let shift = c.size().cast() * HALF_EDGE_LENGTH;
                let trans_shift = camera.transform_delta(shift);
                let center = p_t + trans_shift;

                let r = graphics::Rect {
                    x: center.x,
                    y: center.y,
                    w: trans_size.x,
                    h: trans_size.y
                };

                graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;

                let chip_str = format!("{:?}", chip_id);
                let chip_text = graphics::Text::new(ctx, &chip_str, font)?;
                let chip_text_pos = graphics::Point::new(center.x, center.y);
                chip_text.draw(ctx, chip_text_pos, 0.0)?;
            }
        }
        
        Ok(())
    }

    pub fn draw_components(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        camera: &Camera,
        circuit: &Circuit,
    ) -> GameResult<()> {
        for (ref _id, ref c) in circuit.components().iter() {
            self.draw_component(ctx, font, camera, c, DrawMode::Real)?;
        }

        Ok(())
    }

    pub fn draw_flow(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        camera: &Camera,
        circuit: &Circuit,
        state: &flow::State
    ) -> GameResult<()> {
        for (&id, ref c) in circuit.components().iter() {
            for (cell_index, _pos) in c.cells.iter().enumerate() {
                if let Element::Input { .. } = c.element {
                    continue;
                }
                if let Element::Output { .. } = c.element {
                    continue;
                }
            
                let cell_id = (id, cell_index);
                let node_index = state.graph.node_index(cell_id);
                let cell = state.flow.node(node_index);
                let is_bridge_inner =
                    c.element == Element::Bridge && cell_index == 1;

                let p = c.cells[cell_index].cast();
                let p_t = camera.transform(p * EDGE_LENGTH);
                let size = camera.transform_distance(EDGE_LENGTH * 0.45);
                let size =
                    if is_bridge_inner {
                        size / 2.0
                    } else {
                        size
                    };
                let size =
                    if let &Element::Switch(_) = &c.element {
                        size / 2.0
                    } else {
                        size
                    };

                let pressure = cell.pressure as f32;
                graphics::set_color(ctx,
                    graphics::Color::new(1.0 * (pressure/100.0),
                                         0.0,
                                         1.0 * (1.0 - pressure/100.0),
                                         1.0))?;
                
                if c.element == Element::Power {
                    let dir = Dir::Left.rotate_cw_n(c.rotation_cw);
                    let orth_dir = dir.rotate_cw();
                    let left = c.pos.cast() + dir.delta().cast() / 4.0;
                    let x = left + orth_dir.delta().cast() / 4.0;
                    let y = left + orth_dir.invert().delta().cast() / 4.0;
                    let z = c.pos.cast() + dir.invert().delta().cast() / 4.0;
                    
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
                } else if c.element != Element::Source && c.element != Element::Sink {
                    let r = graphics::Rect {
                        x: p_t.x,
                        y: p_t.y,
                        w: size,
                        h: size
                    };

                    graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;
                    
                    if is_bridge_inner {
                        self.draw_component(ctx, font, camera, c, DrawMode::Real)?;
                    }
                } else {
                    graphics::circle(ctx, graphics::DrawMode::Fill,
                                     graphics::Point { x: p_t.x, y: p_t.y },
                                     size / 2.0,
                                     50)?;
                }
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
            
            /*if edge.enabled {
                let a_t = camera.transform(a_p.cast() * EDGE_LENGTH);
                let b_t = camera.transform(b_p.cast() * EDGE_LENGTH);

                let a_point = graphics::Point::new(a_t.x, a_t.y);
                let b_point = graphics::Point::new(b_t.x, b_t.y);

                graphics::line(ctx, &vec![a_point, b_point])?;
            }*/
            
            let mut percent = edge.flow.abs() as f32 / 10.0; // TODO
            if percent > 1.0 {
                percent = 1.0;
            }
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
                if edge.flow > 0.0 {
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
    
    pub fn draw_flow_debug(
        &self,
        ctx: &mut Context,
        font: &graphics::Font,
        camera: &Camera,
        circuit: &Circuit,
        state: &flow::State
    ) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
    
        for (&id, ref c) in circuit.components().iter() {
            for (cell_index, _pos) in c.cells.iter().enumerate() {
                let cell_id = (id, cell_index);
                let node_index = state.graph.node_index(cell_id);
                let cell = state.flow.node(node_index);

                let p = c.cells[cell_index].cast() - Vector2::new(0.0, 0.5);
                let p_t = camera.transform(p * EDGE_LENGTH);
                
                let s = format!("{:.2}", cell.load);
                let text = graphics::Text::new(ctx, &s, &font)?;
                text.draw(ctx, graphics::Point { x: p_t.x, y: p_t.y }, 0.0)?;
            }
        }
        
        Ok(())
    }
}

