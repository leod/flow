extern crate ggez;
extern crate cgmath;
extern crate floating_duration;

mod types;
mod grid;
mod component;
mod display;
mod circuit;
mod hud;

use std::time::Duration;
use std::collections::HashMap;

use cgmath::Vector2;

use floating_duration::TimeAsFloat;

use ggez::conf;
use ggez::event::{self, MouseButton, MouseState, Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics;

use types::{Dir, Coords};
use circuit::Circuit;
use display::{Display, Camera};
use grid::Grid;
use component::Component;
use hud::{Hud, Input};

struct MainState {
    font: graphics::Font,
    text: graphics::Text,
    frames: usize,
	hud: Hud,

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
            hud: Hud::new(),
            circuit: Circuit::new(Grid::new(), HashMap::new()),
            display: Display::new(),
            camera: Camera::new(),
            camera_delta: Vector2::new(0.0, 0.0),
            camera_speed: 10.0
        };
        Ok(s)
    }
}

impl MainState {
    fn input_event(&mut self, input: &Input) {
        self.hud.input_event(input);
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
        self.display.draw_grid_edges(ctx, &self.camera, &self.circuit.grid)?;

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input_event(&Input::MouseButtonDown {
            button: button,
            x: x, 
            y: y
        });
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input_event(&Input::MouseButtonUp {
            button: button,
            x: x,
            y: y
        });
    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        self.input_event(&Input::MouseWheel {
            x: x,
            y: y
        });

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
        };
    }

    fn mouse_motion_event(
        &mut self, 
        state: MouseState, 
        x: i32, 
        y: i32, 
        xrel: i32, 
        yrel: i32
    ) {
        self.input_event(&Input::MouseMotion {
            state: state,
            x: x,
            y: y,
            xrel: xrel,
            yrel: yrel
        });
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.input_event(&Input::KeyDown {
            keycode: keycode,
            keymod: keymod,
            repeat: repeat
        });

        match keycode {
            Keycode::Left => self.camera_delta.x = -1.0,
            Keycode::Right => self.camera_delta.x = 1.0,
            Keycode::Up => self.camera_delta.y = -1.0,
            Keycode::Down => self.camera_delta.y = 1.0,
            _ => ()
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.input_event(&Input::KeyUp {
            keycode: keycode,
            keymod: keymod,
            repeat: repeat
        });

        match keycode {
            Keycode::Left => self.camera_delta.x = 0.0,
            Keycode::Right => self.camera_delta.x = 0.0,
            Keycode::Up => self.camera_delta.y = 0.0,
            Keycode::Down => self.camera_delta.y = 0.0,
            _ => ()
        }
    }

}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("flow", "leod", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    for x in 0..100 {
        for y in 0..100 {
            if x % 2 == 0 && y % 3 == 0 {
                state.circuit.grid.set_edge(Coords::new(x, y), Dir::Right,
                    grid::Edge { layer: grid::Layer::Ground });
            }

            if x % 2 == 1 {
                state.circuit.grid.set_edge(Coords::new(x, y), Dir::Down,
                    grid::Edge { layer: grid::Layer::Ground });
            }
        }
    }

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

