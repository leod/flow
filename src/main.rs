extern crate ggez;
extern crate cgmath;
extern crate floating_duration;

mod types;
mod grid;
mod component;
mod display;
mod circuit;
mod hud;
mod camera;
mod camera_input;
mod input;

use std::time::Duration;
use std::collections::HashMap;

use floating_duration::TimeAsFloat;

use ggez::conf;
use ggez::event::{self, MouseButton, MouseState, Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics;

use types::Dir;
use circuit::Circuit;
use display::Display;
use grid::Grid;
use hud::Hud;
use camera::Camera;
use camera_input::CameraInput;
use input::Input;

struct MainState {
    circuit: Circuit,

    frames: usize,

    hud: Hud,
    display: Display,

    camera: Camera,
    camera_input: CameraInput,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        let s = MainState {
            circuit: Circuit::new(Grid::new(), HashMap::new()),
            frames: 0,
            hud: Hud::new(ctx)?,
            display: Display::new(),
            camera: Camera::new(ctx.conf.window_width, ctx.conf.window_height, 50.0),
            camera_input: CameraInput::new(10.0)
        };
        Ok(s)
    }
}

impl MainState {
    fn input_event(&mut self, input: &Input) {
        self.hud.input_event(&mut self.circuit, &self.camera, input);
        self.camera_input.input_event(&mut self.camera, input);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let dt_s = dt.as_fractional_secs() as f32;

        self.camera_input.update(&mut self.camera, dt_s);
        self.hud.update(ctx, &mut self.circuit, &self.camera, dt_s);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.display.draw_grid_edges(ctx, &self.camera, &self.circuit.grid)?;
        self.display.draw_grid_points(ctx, &self.camera, &self.circuit.grid)?;
        self.hud.draw(ctx, &self.camera)?;

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
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.input_event(&Input::KeyUp {
            keycode: keycode,
            keymod: keymod,
            repeat: repeat
        });
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("flow", "leod", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    /*for x in 0..100 {
        for y in 0..100 {
            if x % 2 == 0 && y % 3 == 0 {
                state.circuit.grid.set_edge(grid::Coords::new(x, y), Dir::Right,
                    grid::Edge { layer: grid::Layer::Ground });
            }

            if x % 2 == 1 {
                state.circuit.grid.set_edge(grid::Coords::new(x, y), Dir::Down,
                    grid::Edge { layer: grid::Layer::Ground });
            }
        }
    }*/

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

