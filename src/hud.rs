use std::cmp;
use std::iter::{once, Iterator};
use std::io::{self, Write};

use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics::{self, Drawable};

use types::{Dir, Axis};
use input::{self, Input};
use camera::Camera;
use grid::{self, Grid};
use component::{Component, Element};
use circuit::Circuit;
use display;

#[derive(PartialEq, Eq, Clone, Debug)]
enum State {
    Initial,
    Drawing {
        last_grid_coords: grid::Coords,
        axis_lock: Option<Axis>,
        undo: Vec<Action>
    },
    PlaceElement(Element)
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Action {
    SetPoint(grid::Coords, Option<grid::Point>),
    SetEdge(grid::Coords, Dir, Option<grid::Edge>),
    Compound(Vec<Action>)
}

pub struct Hud {
    font: graphics::Font,

    state: State,
    undo: Vec<Action>,
    redo: Vec<Action>,

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

impl Action {
    fn perform(self, circuit: &mut Circuit) -> Action {
        match self {
            Action::SetPoint(c, p_new) => {
                let p = circuit.grid.get_point(c);
                let undo = Action::SetPoint(c, p);
                circuit.grid.set_point_option(c, p_new);
                undo
            }
            Action::SetEdge(c, dir, e_new) => {
                let e =  circuit.grid.get_edge(c, dir);
                let undo = Action::SetEdge(c, dir, e);
                circuit.grid.set_edge_option(c, dir, e_new);
                undo
            }
            Action::Compound(actions) => {
                let mut undo = actions.into_iter()
                                      .map(|action| { action.perform(circuit) })
                                      .collect::<Vec<_>>();
                undo.reverse();
                Action::Compound(undo) 
            }
        }
    }
}

impl Hud {
    pub fn new(ctx: &mut Context) -> GameResult<Hud> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 10)?;

        let h = Hud {
            font: font,
            state: State::Initial,
            undo: Vec::new(),
            redo: Vec::new(),
            mouse_x: ctx.conf.window_width as i32 / 2,
            mouse_y: ctx.conf.window_height as i32 / 2,
            hold_control: false,
            grid_coords: grid::Coords::new(0, 0),
        };
        Ok(h)
    }

    fn change_state(&mut self, new_state: State) {
        self.leave_state();
        self.state = new_state;
    }

    fn push_undo(&mut self, undo_action: Action) {
        self.undo.push(undo_action);

        // After a user action is performed, clear redo
        self.redo.clear();
    }

    fn leave_state(&mut self) {
        let undo_action = match self.state {
            State::Initial => None,
            State::Drawing { ref mut undo, .. } => {
                undo.reverse();
                Some(Action::Compound(undo.clone()))
            },
            State::PlaceElement(_) => None,
        };

        if let Some(u) = undo_action {
            self.push_undo(u);
        }
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
                    input::Keycode::Z => {
                        if self.hold_control == true {
                            if let Some(undo_action) = self.undo.pop() {
                                let redo_action = undo_action.perform(circuit);
                                self.redo.push(redo_action);
                            }
                        }
                    }
                    input::Keycode::Y => {
                        if self.hold_control == true {
                            if let Some(redo_action) = self.redo.pop() {
                                let undo_action = redo_action.perform(circuit);
                                self.undo.push(undo_action);
                            }
                        }
                    }
                    input::Keycode::Num1 => {
                        self.change_state(State::Initial);
                    }
                    input::Keycode::Num2 => {
                        self.change_state(State::PlaceElement(Element::Source));
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
        let grid_coords = screen_to_grid_coords(camera, x, y);

        match self.state {
            State::Initial => {
                match button {
                    input::MouseButton::Left => {
                        let action = Action::SetPoint(grid_coords,
                                                      Some(grid::Point::Node));
                        let undo =
                            if circuit.grid.get_point(grid_coords).is_none() {
                                let undo_action = action.perform(circuit);
                                vec![undo_action]
                            } else {
                                vec![]
                            };

                        let new_state = State::Drawing {
                            last_grid_coords: grid_coords,
                            axis_lock: None,
                            undo: undo
                        };
                        self.change_state(new_state);
                    }
                    input::MouseButton::Right => {
                        let action = Action::SetPoint(grid_coords, None);
                        let undo_action = action.perform(circuit);
                        self.push_undo(undo_action);
                    }
                    _ => {}
                }
            }
            State::Drawing { .. }  => {
                match button {
                    input::MouseButton::Right => {
                        self.change_state(State::Initial);
                    }
                    _ => {}
                }
            }
            State::PlaceElement(element) => {
                
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
                        self.change_state(State::Initial);
                    }
                    _ => {}
                }
            },
            State::PlaceElement(_) => {}
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
            State::Drawing { ref mut last_grid_coords,
                             ref mut axis_lock,
                             ref mut undo } => {
                if !self.hold_control {
                    *axis_lock = None;
                }

                let locked_coords = match *axis_lock {
                    Some(Axis::Horizontal) =>
                        grid::Coords::new(self.grid_coords.x,
                                          last_grid_coords.y),
                    Some(Axis::Vertical) =>
                        grid::Coords::new(last_grid_coords.x,
                                          self.grid_coords.y),
                    None =>
                        self.grid_coords 
                };

                if locked_coords != *last_grid_coords {
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
                                let action =
                                    Action::SetPoint(c, Some(grid::Point::Node));
                                let undo_action = action.perform(circuit);
                                undo.push(undo_action);
                            }

                            if let Some(p) = prev_c {
                                let dir = Dir::from_coords(p, c);
                                let edge = grid::Edge {
                                    layer: grid::Layer::Ground
                                };

                                let action = Action::SetEdge(p, dir, Some(edge));
                                let undo_action = action.perform(circuit);
                                undo.push(undo_action);

                                if self.hold_control && axis_lock.is_none() {
                                    *axis_lock = Some(dir.to_axis());
                                }
                            }
                        }

                        prev_c = Some(c);
                    }
                }

                *last_grid_coords = locked_coords;
            }
            State::PlaceElement(_) => {}
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

        let state_str = format!("{:?}", self.state);
        let state_text = graphics::Text::new(ctx, &state_str, &self.font)?;
        state_text.draw(ctx, graphics::Point::new(10.0 + state_text.width() as f32 / 2.0, 10.0), 0.0)?;

        Ok(())
    }
}
