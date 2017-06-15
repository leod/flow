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

        let p_t = (p - self.position) * self.zoom + shift;
        p_t
    }

    pub fn untransform(self: &Camera, p_t: Vector2<f32>) -> Vector2<f32> {
        let shift = 0.5 * Vector2::new(self.window_width as f32,
                                       self.window_height as f32);

        let p = (p_t - shift) / self.zoom + self.position;
        p
    }

    pub fn transform_distance(self: &Camera, r: f32) -> f32 {
        r * self.zoom
    }
}
