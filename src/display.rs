use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use types::PosDir;
use grid::{self, Grid};
use camera::Camera;

pub struct Display {
   edge_length: f32 
}

impl Display {
    pub fn new() -> Display {
        Display {
            edge_length: 1.0
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
            let start = graphics::Point {
                x: c.x as f32 * self.edge_length,
                y: c.y as f32 * self.edge_length
            };

            let end = match dir {
                PosDir::Right =>
                    graphics::Point {
                        x: (c.x+1) as f32 * self.edge_length,
                        y: c.y as f32 * self.edge_length
                    },
                PosDir::Down =>
                    graphics::Point {
                        x: c.x as f32 * self.edge_length,
                        y: (c.y+1) as f32 * self.edge_length
                    },
                _ =>
                    panic!("unexpected edge orientation")
            };

            graphics::line(ctx,
                &vec![camera.transform_point(start), camera.transform_point(end)])?;
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
            let p_t = camera.transform(c.cast() * self.edge_length);

            let r = graphics::Rect {
                x: p_t.x,
                y: p_t.y,
                w: camera.transform_distance(0.5),
                h: camera.transform_distance(0.5)
            };

            graphics::rectangle(ctx, graphics::DrawMode::Line, r);
        }

        Ok(())
    }
}

