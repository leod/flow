use cgmath::Vector2;
use ggez::graphics;

#[derive(Clone, Debug)]
pub struct Camera {
    window_width: u32,
    window_height: u32,
    pub position: Vector2<f32>,
    pub zoom: f32
}

impl Camera {
    pub fn new(window_width: u32, window_height: u32, zoom: f32) -> Camera {
        Camera {
            window_width: window_width,
            window_height: window_height,
            position: Vector2::new(0.0, 0.0),
            zoom: zoom
        }
    }

    pub fn transform(self: &Camera, p: Vector2<f32>) -> Vector2<f32> {
        let shift = 0.5 * Vector2::new(self.window_width as f32,
                                       self.window_height as f32);

        (p - self.position) * self.zoom + shift
    }

    pub fn transform_point(self: &Camera, p: graphics::Point) -> graphics::Point {
        let c = self.transform(Vector2::<f32>::new(p.x, p.y));

        graphics::Point {
           x: c.x,
           y: c.y 
        }
    }
}
