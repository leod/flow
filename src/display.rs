use cgmath::{Zero, ElementWise, Vector2};

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

        let descr = component.element.descr();
        let size = component.size();

        for &(dir, steps) in descr.edge_points.iter() {
            let dir_rot = dir.rotate_cw_n(component.rotation); 

            println!("dir_rot: {:?}", dir_rot);

            let origin =
                if dir_rot.is_pos() {
                    component.top_left_pos +
                        dir_rot.apply(Vector2::zero()).mul_element_wise(size - Vector2::new(1, 1))
                } else {
                    component.top_left_pos
                };

            let a = dir_rot.rotate_cw().apply_n(origin, steps);
            let b = dir_rot.apply(a);

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
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        for (ref _id, ref c) in circuit.components().iter() {
            let p_t = camera.transform(c.top_left_pos.cast() * EDGE_LENGTH);

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
                Element::Source => {
                    let size = (c.size().cast() - Vector2::new(0.5, 0.5))
                        * EDGE_LENGTH;
                    let shift = (c.size().cast() - Vector2::new(1.0, 1.0))
                        * HALF_EDGE_LENGTH;
                    let trans_size = camera.transform_delta(size);
                    let trans_shift = camera.transform_delta(shift);
                    let center = p_t + trans_shift;

                    let r = graphics::Rect {
                        x: center.x,
                        y: center.y,
                        w: trans_size.x,
                        h: trans_size.y
                    };

                    self.draw_component_edge_points(ctx, camera, c);

                    graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
                    /*graphics::circle(ctx, graphics::DrawMode::Line,
                                     graphics::Point { x: center.x, y: center.y },
                                     (trans_size / 2.0).magnitude(),
                                     10);*/
                }
                _ => {}
            }
        }

        Ok(())
    }
}

