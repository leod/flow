extern crate ggez;
extern crate cgmath;
extern crate floating_duration;
extern crate rulinalg;
extern crate rand;
extern crate sdl2;

mod types;
mod display;
mod circuit;
mod hud;
mod camera;
mod camera_input;
mod input;
mod canon_map;
mod flow;
mod graph;
mod level;
mod test_level;
//#[cfg(test)] mod tests;

use std::time::Duration;

use floating_duration::TimeAsFloat;

use ggez::conf;
use ggez::event::{self, MouseButton, MouseState, Mod};
use ggez::{GameResult, Context};
use ggez::graphics;

use circuit::{ChipDb, Circuit};
use display::Display;
use hud::Hud;
use camera::Camera;
use camera_input::CameraInput;
use input::{Input, Keycode};
use level::{Level, LevelState};

struct MainState {
    chip_db: ChipDb,

    circuit: Circuit,

    level: Level,
    level_state: Option<LevelState>,

    frames: usize,

    hud: Hud,
    display: Display,

    camera: Camera,
    camera_input: CameraInput,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_background_color(
            ctx,
            graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        );
        let level = test_level::test_level();
        let s = MainState {
            chip_db: ChipDb::init(10),
            circuit: level.new_circuit(),
            level: level,
            level_state: None,
            frames: 0,
            hud: Hud::new(ctx)?,
            display: Display::new(),
            camera: Camera::new(
                ctx.conf.window_width,
                ctx.conf.window_height,
                30.0,
            ),
            camera_input: CameraInput::new(8.0),
        };
        Ok(s)
    }

    fn input_event(&mut self, input: &Input) {
        // Only allow changing the circuit when not simulating
        if self.level_state.is_none() {
            self.hud.input_event(
                &mut self.circuit,
                &mut self.chip_db,
                &self.camera,
                input,
            );
        }

        self.camera_input.input_event(&mut self.camera, input);

        match input {
            &Input::KeyDown {
                keycode: Keycode::Space,
                keymod: _,
                repeat: _,
            } => {
                self.level_state = match &self.level_state {
                    &Some(_) => None,
                    &None => {
                        // Start simulation
                        let unfolded_circuit =
                            self.circuit.unfold(&self.chip_db);
                        if let Some(circuit) = unfolded_circuit {
                            Some(self.level.new_state(&circuit))
                        } else {
                            println!("Circuit is cyclic");
                            None
                        }
                    }
                };
                if self.level_state.is_some() {
                    self.hud.switch_chip(&None);
                }
            }
            &Input::KeyDown {
                keycode,
                keymod: _,
                repeat: _,
            } => {
                if keycode == Keycode::T {
                    let finished = if let &mut Some(ref mut level_state) =
                        &mut self.level_state
                    {
                        if let Some(outcome) = level_state.time_step() {
                            println!("level outcome: {:?}", outcome);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if finished {
                        self.level_state = None;
                    }
                }
            }
            _ => {}
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let dt_s = dt.as_fractional_secs() as f32;

        self.camera_input.update(&mut self.camera, dt_s);
        self.hud.update(
            ctx,
            &mut self.circuit,
            &mut self.chip_db,
            &self.camera,
            dt_s,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let circuit = self.hud.cur_circuit(&self.circuit, &self.chip_db);
        self.display.draw_grid_edges(ctx, &self.camera, circuit)?;
        self.display.draw_components(
            ctx,
            &self.hud.font,
            &self.camera,
            circuit,
        )?;

        if let &Some(ref level_state) = &self.level_state {
            if self.hud.cur_chip_id().is_none() {
                let flow = &level_state.flow;
                self.display.draw_flow(
                    ctx,
                    &self.hud.font,
                    &self.camera,
                    &self.circuit,
                    flow,
                )?;
                //self.display.draw_flow_debug(ctx, &self.hud.font, &self.camera, &self.circuit, flow)?;
            }
        } else {
            self.hud.draw(
                ctx,
                &self.circuit,
                &self.chip_db,
                &self.camera,
                &self.display,
            )?;
        }

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            //println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input_event(&Input::MouseButtonDown {
            button: button,
            x: x,
            y: y,
        });
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input_event(&Input::MouseButtonUp {
            button: button,
            x: x,
            y: y,
        });
    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        self.input_event(&Input::MouseWheel { x: x, y: y });
    }

    fn mouse_motion_event(
        &mut self,
        state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        self.input_event(&Input::MouseMotion {
            state: state,
            x: x,
            y: y,
            xrel: xrel,
            yrel: yrel,
        });
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.input_event(&Input::KeyDown {
            keycode: keycode,
            keymod: keymod,
            repeat: repeat,
        });
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.input_event(&Input::KeyUp {
            keycode: keycode,
            keymod: keymod,
            repeat: repeat,
        });
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("flow", "leod", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
