use cgmath::Vector2;

use input::{Input, Keycode};
use camera::Camera;

pub struct CameraInput {
    delta: Vector2<f32>,
    speed: f32,
}

impl CameraInput {
    pub fn new(speed: f32) -> CameraInput {
        CameraInput {
            delta: Vector2::new(0.0, 0.0),
            speed: speed
        }
    }

    pub fn input_event(&mut self, camera: &mut Camera, input: &Input) {
        match input {
            &Input::MouseWheel { x, y } => {
                if y < 0 && camera.zoom > 1.0 {
                    camera.zoom += 0.1 * y as f32;
                    if camera.zoom < 1.0 {
                        camera.zoom = 1.0;
                    }
                }
                if y > 0 && camera.zoom < 10.0 {
                    camera.zoom += 0.1 * y as f32;
                    if camera.zoom > 10.0 {
                       camera.zoom = 10.0;
                    }
                };
            }
            &Input::KeyDown { keycode, keymod, repeat } => {
                match keycode {
                    Keycode::Left => self.delta.x = -1.0,
                    Keycode::Right => self.delta.x = 1.0,
                    Keycode::Up => self.delta.y = -1.0,
                    Keycode::Down => self.delta.y = 1.0,
                    _ => ()
                }
            }
            &Input::KeyUp { keycode, keymod, repeat } => {
                match keycode {
                    Keycode::Left => self.delta.x = 0.0,
                    Keycode::Right => self.delta.x = 0.0,
                    Keycode::Up => self.delta.y = 0.0,
                    Keycode::Down => self.delta.y = 0.0,
                    _ => ()
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, camera: &mut Camera, dt_s: f32) {
        camera.position += self.delta * self.speed * dt_s;
    }
}
