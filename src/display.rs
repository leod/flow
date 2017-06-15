use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use types::PosDir;
use grid::{self, Grid};
use camera::Camera;

pub const EDGE_LENGTH: f32 = 1.5;

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
        grid: &Grid
    ) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        for (&(c, dir), &edge) in grid.iter_edges() {
            let a = camera.transform(c.cast() * EDGE_LENGTH);
            let b = camera.transform(dir.apply(c).cast() * EDGE_LENGTH);

            let p_a = graphics::Point::new(a.x, a.y);
            let p_b = graphics::Point::new(b.x, b.y);

            graphics::line(ctx, &vec![p_a, p_b]);
        }

        Ok(())
    }

    pub fn draw_grid_points(
        self: &Display,
        ctx: &mut Context,
        camera: &Camera,
        grid: &Grid
    ) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

        for (&c, &point) in grid.iter_points() {
            let p_t = camera.transform(c.cast() * EDGE_LENGTH);

            let r = graphics::Rect {
                x: p_t.x,
                y: p_t.y,
                w: camera.transform_distance(EDGE_LENGTH / 2.0),
                h: camera.transform_distance(EDGE_LENGTH / 2.0)
            };

            graphics::rectangle(ctx, graphics::DrawMode::Line, r);
        }

        Ok(())
    }
}

