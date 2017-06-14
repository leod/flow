use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use types::PosDir;
use grid::{self, Grid};

#[derive(Clone, Debug)]
pub struct Camera {
   pub position: Vector2<f32>,
   pub zoom: f32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2::<f32>::new(0.0, 0.0),
            zoom: 1.0
        }
    }

    pub fn transform(self: &Camera, p: Vector2<f32>) -> Vector2<f32> {
        (p - self.position) * self.zoom
    }

    pub fn transform_point(self: &Camera, p: graphics::Point) -> graphics::Point {
        let c = self.transform(Vector2::<f32>::new(p.x, p.y));

        graphics::Point {
           x: c.x,
           y: c.y 
        }
    }
}

pub struct Display {
   edge_length: f32 
}

impl Display {
    pub fn new() -> Display {
        Display {
            edge_length: 1.0
        }
    }

    pub fn draw_grid_edges(self: &Display, ctx: &mut Context, camera: &Camera,
                           grid: &Grid) -> GameResult<()> {
        for (&(c, dir), &edge) in grid.iter_edges() {
            let start = graphics::Point {
                x: c.x as f32 * self.edge_length,
                y: c.y as f32 * self.edge_length
            };

            let end = match dir {
                PosDir::PosRight =>
                    graphics::Point {
                        x: (c.x+1) as f32 * self.edge_length,
                        y: c.y as f32 * self.edge_length
                    },
                PosDir::PosDown =>
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
}

