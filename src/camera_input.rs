use cgmath::Vector2;

use input::{Input, Keycode};
use camera::Camera;

pub struct CameraInput {
    min_zoom: f32,
    max_zoom: f32,
    scroll_speed: f32,
    zoom_speed: f32,

    delta: Vector2<f32>,
}

impl CameraInput {
    pub fn new(scroll_speed: f32) -> CameraInput {
        CameraInput {
            min_zoom: 1.0,
            max_zoom: 100.0,
            scroll_speed: scroll_speed,
            zoom_speed: 1.0,
            delta: Vector2::new(0.0, 0.0),
        }
    }

    pub fn input_event(&mut self, camera: &mut Camera, input: &Input) {
        match input {
            &Input::MouseWheel { x: _, y } => {
                // Zoom out
                if y < 0 && camera.zoom > self.min_zoom {
                    camera.zoom += self.zoom_speed * y as f32;
                    if camera.zoom < self.min_zoom {
                        camera.zoom = self.min_zoom;
                    }
                }

                // Zoom in
                if y > 0 && camera.zoom < self.max_zoom {
                    camera.zoom += self.zoom_speed * y as f32;
                    if camera.zoom > self.max_zoom {
                        camera.zoom = self.max_zoom;
                    }
                };
            }
            &Input::KeyDown {
                keycode,
                keymod: _,
                repeat: _,
            } => {
                match keycode {
                    Keycode::Left | Keycode::A => self.delta.x = -1.0,
                    Keycode::Right | Keycode::D => self.delta.x = 1.0,
                    Keycode::Up | Keycode::W => self.delta.y = -1.0,
                    Keycode::Down | Keycode::S => self.delta.y = 1.0,
                    _ => (),
                }
            }
            &Input::KeyUp {
                keycode,
                keymod: _,
                repeat: _,
            } => {
                match keycode {
                    Keycode::Left | Keycode::A => self.delta.x = 0.0,
                    Keycode::Right | Keycode::D => self.delta.x = 0.0,
                    Keycode::Up | Keycode::W => self.delta.y = 0.0,
                    Keycode::Down | Keycode::S => self.delta.y = 0.0,
                    _ => (),
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt_s: f32) {
        camera.position += self.delta * self.scroll_speed * dt_s;
    }
}
