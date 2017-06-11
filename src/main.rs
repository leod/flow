extern crate ggez;
extern crate cgmath;
extern crate floating_duration;

mod types;
mod grid;
mod component;
mod display;
mod circuit;

use std::time::Duration;
use std::collections::HashMap;

use cgmath::Vector2;

use floating_duration::TimeAsFloat;

use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics;

use types::{Orientation, Coords};
use circuit::Circuit;
use display::{Display, Camera};
use grid::Grid;
use component::Component;

struct MainState {
    font: graphics::Font,
    text: graphics::Text,
    frames: usize,

    circuit: Circuit,

    display: Display,

    camera: Camera,
    camera_delta: Vector2<f32>,
    camera_speed: f32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState {
            font: font,
            text: text,
            frames: 0,
            circuit: Circuit::new(Grid::new(100, 100), HashMap::new()),
            display: Display::new(),
            camera: Camera::new(),
            camera_delta: Vector2::new(0.0, 0.0),
            camera_speed: 10.0
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        self.camera.position += self.camera_delta *
                                self.camera_speed *
                                dt.as_fractional_secs() as f32;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // Drawables are drawn from their center.
        /*let dest_point = graphics::Point::new(self.text.width() as f32 / 2.0 + 10.0,
                                              self.text.height() as f32 / 2.0 + 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;*/

        self.display.draw_grid_edges(ctx, &self.camera, &self.circuit.grid)?;

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => self.camera_delta.x = -1.0,
            Keycode::Right => self.camera_delta.x = 1.0,
            Keycode::Up => self.camera_delta.y = -1.0,
            Keycode::Down => self.camera_delta.y = 1.0,
            _ => ()
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left => self.camera_delta.x = 0.0,
            Keycode::Right => self.camera_delta.x = 0.0,
            Keycode::Up => self.camera_delta.y = 0.0,
            Keycode::Down => self.camera_delta.y = 0.0,
            _ => ()
        }
    }

    fn mouse_wheel_event(&mut self, _x: i32, y: i32) {
        if y < 0 && self.camera.zoom > 1.0 {
            self.camera.zoom += 0.1 * y as f32;
            if self.camera.zoom < 1.0 {
                self.camera.zoom = 1.0;
            }
        }
        if y > 0 && self.camera.zoom < 10.0 {
            self.camera.zoom += 0.1 * y as f32;
            if self.camera.zoom > 10.0 {
                self.camera.zoom = 10.0;
            }
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    for x in 0..state.circuit.grid.width()-2 {
        for y in 0..state.circuit.grid.height()-2 {
            if x % 2 == 0 && y % 3 == 0 {
                state.circuit.grid.set_edge(Coords::new(x, y), Orientation::Right,
                    grid::Edge::Connected(grid::Layer::Ground));
            }

            if x % 2 == 1 {
                state.circuit.grid.set_edge(Coords::new(x, y), Orientation::Down,
                    grid::Edge::Connected(grid::Layer::Ground));
            }
        }
    }

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

