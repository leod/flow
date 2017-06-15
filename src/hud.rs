use std::cmp;
use std::iter::once;
use std::io::{self, Write};

use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use types::{Dir, Axis};
use input::{self, Input};
use camera::Camera;
use grid::{self, Grid};
use circuit::Circuit;
use display;

enum State {
    Initial,
    Drawing {
        last_grid_coords: grid::Coords,
        axis_lock: Option<Axis>
    }
}

enum Action {

}

pub struct Hud {
    font: graphics::Font,

    state: State,

    mouse_x: i32,
    mouse_y: i32,
    hold_control: bool,

    grid_coords: grid::Coords,
}

fn screen_to_grid_coords(camera: &Camera, x: i32, y: i32) -> grid::Coords {
    let mouse_p_t = Vector2::new(x as f32, y as f32);
    let mouse_p = camera.untransform(mouse_p_t) / display::EDGE_LENGTH;
    let g_x = mouse_p.x.round() as isize;
    let g_y = mouse_p.y.round() as isize;

    grid::Coords::new(g_x, g_y)
}

impl Hud {
    pub fn new(ctx: &mut Context) -> GameResult<Hud> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        let h = Hud {
            font: font,
            state: State::Initial,
            mouse_x: ctx.conf.window_width as i32 / 2,
            mouse_y: ctx.conf.window_height as i32 / 2,
            hold_control: false,
            grid_coords: grid::Coords::new(0, 0),
        };
        Ok(h)
    }

	pub fn input_event(
        &mut self,
        circuit: &mut Circuit,
        camera: &Camera,
        input: &Input
    ) {
        match input {
            &Input::MouseMotion { state: _, x, y, xrel: _, yrel: _ } => {
                self.mouse_x = x;
                self.mouse_y = y;

                self.mouse_motion_event(circuit, camera, x, y);
            }
            &Input::MouseButtonDown { button, x, y } => {
                self.mouse_button_down_event(circuit, camera, button, x, y);
            }
            &Input::MouseButtonUp { button, x, y } => {
                self.mouse_button_up_event(circuit, camera, button, x, y);
            }
            &Input::KeyDown { keycode, keymod: _, repeat: _ } => {
                match keycode {
                    input::Keycode::LCtrl => {
                        self.hold_control = true;
                    }
                    _ => {}
                }
            }
            &Input::KeyUp { keycode, keymod: _, repeat: _ } => {
                match keycode {
                    input::Keycode::LCtrl => {
                        self.hold_control = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn mouse_motion_event(
        &mut self,
        circuit: &mut Circuit,
        camera: &Camera,
        mouse_x: i32,
        mouse_y: i32
    ) {
    }

    fn mouse_button_down_event(
        &mut self,
        circuit: &mut Circuit,
        camera: &Camera,
        button: input::MouseButton,
        x: i32,
        y: i32
    ) {
        match self.state {
            State::Initial => {
                match button {
                    input::MouseButton::Left => {
                        let grid_coords = screen_to_grid_coords(camera, x, y);
                        circuit.grid.set_point(grid_coords, grid::Point::Node);

                        self.state = State::Drawing {
                            last_grid_coords: grid_coords,
                            axis_lock: None
                        };
                    }
                    input::MouseButton::Right => {
                        let grid_coords = screen_to_grid_coords(camera, x, y);
                        circuit.grid.remove_point(grid_coords);
                    }
                    _ => {}
                }
            }
            State::Drawing { .. }  => {
                match button {
                    input::MouseButton::Right => {
                        self.state = State::Initial;
                    }
                    _ => {}
                }
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        circuit: &mut Circuit,
        camera: &Camera,
        button: input::MouseButton,
        _x: i32,
        _y: i32
    ) {
        match self.state {
            State::Initial => {}
            State::Drawing { .. }  => {
                match button {
                    input::MouseButton::Left => {
                        self.state = State::Initial;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(
        &mut self,
        _ctx: &mut Context,
        circuit: &mut Circuit,
        camera: &Camera,
        _dt_s: f32
    ) {
        self.grid_coords = screen_to_grid_coords(camera,
                                                 self.mouse_x,
                                                 self.mouse_y);
        match self.state {
            State::Initial => {}
            State::Drawing { last_grid_coords, axis_lock } => {
                let mut new_axis_lock = if !self.hold_control {
                    None
                } else {
                    axis_lock
                };

                let locked_coords = match new_axis_lock {
                    Some(Axis::Horizontal) =>
                        grid::Coords::new(self.grid_coords.x, last_grid_coords.y),
                    Some(Axis::Vertical) =>
                        grid::Coords::new(last_grid_coords.x, self.grid_coords.y),
                    None =>
                        self.grid_coords 
                };

                if locked_coords != last_grid_coords {
                    // We might have jumped more than one grid point.
                    // In this case, draw two lines to get there
                    let min_x = cmp::min(locked_coords.x, last_grid_coords.x);
                    let max_x = cmp::max(locked_coords.x, last_grid_coords.x);
                    let min_y = cmp::min(locked_coords.y, last_grid_coords.y);
                    let max_y = cmp::max(locked_coords.y, last_grid_coords.y);

                    let line_v = (min_x..max_x+1).zip(once(min_y).cycle());
                    let line_h = once(max_x).cycle().zip(min_y..max_y+1);
                    let lines = line_v.chain(line_h);

                    let mut prev_c = None;
                    for (x, y) in lines {
                        let c = grid::Coords::new(x, y);

                        if prev_c.is_none() || Some(c) != prev_c {
                            if circuit.grid.get_point(c).is_none() {
                                circuit.grid.set_point(c, grid::Point::Node);
                            }

                            if let Some(p) = prev_c {
                                let dir = Dir::from_coords(p, c);
                                let edge = grid::Edge {
                                    layer: grid::Layer::Ground
                                };
                                circuit.grid.set_edge(p, dir, edge);

                                if self.hold_control && new_axis_lock.is_none() {
                                    new_axis_lock = Some(dir.to_axis());
                                }
                            }
                        }

                        prev_c = Some(c);
                    }
                }

                self.state = State::Drawing {
                    last_grid_coords: locked_coords,
                    axis_lock: new_axis_lock
                };
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera
    ) -> GameResult<()> {
        let grid_coords_t = camera.transform(
            self.grid_coords.cast() * display::EDGE_LENGTH);

        let r = graphics::Rect {
            x: grid_coords_t.x,
            y: grid_coords_t.y,
            w: camera.transform_distance(display::EDGE_LENGTH / 3.0),
            h: camera.transform_distance(display::EDGE_LENGTH / 3.0)
        };

        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;

        Ok(())
    }
}
