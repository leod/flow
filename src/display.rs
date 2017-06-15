use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use types::Rect;
use circuit::Circuit;
use camera::Camera;
use component::Element;

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
        self: &Display,
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

    pub fn draw_components(
        self: &Display,
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
                    let size = (c.rect().size.cast() - Vector2::new(0.5, 0.5))
                        * EDGE_LENGTH;
                    let shift = (c.rect().size.cast() - Vector2::new(1.0, 1.0))
                        * HALF_EDGE_LENGTH;
                    let trans_size = camera.transform_delta(size);
                    let trans_shift = camera.transform_delta(shift);

                    let r = graphics::Rect {
                        x: p_t.x + trans_shift.x,
                        y: p_t.y + trans_shift.y,
                        w: trans_size.x,
                        h: trans_size.y
                    };
                    
                    graphics::rectangle(ctx, graphics::DrawMode::Line, r)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

