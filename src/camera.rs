use cgmath::Vector2;
use ggez::graphics;

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
