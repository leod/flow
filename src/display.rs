use cgmath::{InnerSpace, Vector2};

use ggez::{GameResult, Context};
use ggez::graphics;

use circuit::Circuit;
use camera::Camera;
use component::{Element, Component};

pub const EDGE_LENGTH: f32 = 1.5;
pub const HALF_EDGE_LENGTH: f32 = EDGE_LENGTH / 2.0;

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

        for (&(c, dir), &_edge) in circuit.edges().iter() {
            let a = camera.transform(c.cast() * EDGE_LENGTH);
            let b = camera.transform(dir.apply(c).cast() * EDGE_LENGTH);

            let p_a = graphics::Point::new(a.x, a.y);
            let p_b = graphics::Point::new(b.x, b.y);

            graphics::line(ctx, &vec![p_a, p_b])?;
        }

        Ok(())
    }

    pub fn draw_component_edge_points(
        &self,
        ctx: &mut Context,
        camera: &Camera,
        component: &Component
    ) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        for &(a, dir) in component.edge_points.iter() {
            let b = dir.apply(a);

            let a_t = camera.transform(a.cast() * EDGE_LENGTH);
            let b_t = camera.transform(b.cast() * EDGE_LENGTH);

            let p_a = graphics::Point::new(a_t.x, a_t.y);
            let p_b = graphics::Point::new(b_t.x, b_t.y);

            graphics::line(ctx, &vec![p_a, p_b])?;
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
            graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

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
                Element::Source | Element::Sink => {
                    let size = (c.size().cast() + Vector2::new(0.5, 0.5))
                        * EDGE_LENGTH;
                    let shift = c.size().cast() * HALF_EDGE_LENGTH;
                    let trans_size = camera.transform_delta(size);
                    let trans_shift = camera.transform_delta(shift);
                    let center = p_t + trans_shift;

                    let r = graphics::Rect {
                        x: center.x,
                        y: center.y,
                        w: trans_size.x,
                        h: trans_size.y
                    };

                    self.draw_component_edge_points(ctx, camera, c)?;

                    graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;

                    graphics::circle(ctx, graphics::DrawMode::Fill,
                                     graphics::Point { x: center.x, y: center.y },
                                     camera.transform_distance(size.x / 2.0),
                                     50);

                    graphics::set_color(ctx,
                                        graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
                    graphics::circle(ctx, graphics::DrawMode::Fill,
                                     graphics::Point { x: center.x, y: center.y },
                                     camera.transform_distance(size.x / 2.0 - 0.05),
                                     50);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

