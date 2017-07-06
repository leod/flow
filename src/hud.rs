use std::cmp;
use std::iter::{once, Iterator};

use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics::{self, Drawable};

use types::{Dir, Axis};
use input::{self, Input};
use camera::Camera;
use circuit::{self, Circuit, Action, Element};
use display::{self, Display};

#[derive(PartialEq, Eq, Clone, Debug)]
enum State {
    Initial,
    Drawing {
        last_grid_coords: circuit::Coords,
        axis_lock: Option<Axis>,
        undo: Vec<Action>
    },
    PlaceElement {
        element: Element,
        rotation_cw: usize
    }
}

pub struct Hud {
    font: graphics::Font,

    state: State,
    undo: Vec<Action>,
    redo: Vec<Action>,

    mouse_x: i32,
    mouse_y: i32,
    hold_control: bool,

    grid_coords: circuit::Coords,
}

fn screen_to_grid_coords(camera: &Camera, x: i32, y: i32) -> circuit::Coords {
    let mouse_p_t = Vector2::new(x as f32, y as f32);
    let mouse_p = camera.untransform(mouse_p_t) / display::EDGE_LENGTH;
    let g_x = mouse_p.x.round() as isize;
    let g_y = mouse_p.y.round() as isize;

    circuit::Coords::new(g_x, g_y)
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
            grid_coords: circuit::Coords::new(0, 0),
        };
        Ok(h)
    }

    fn change_state(&mut self, new_state: State) {
        self.leave_state();
        self.state = new_state;
    }

    fn push_undo(&mut self, undo_action: Action) {
        println!("undo: {:?}", undo_action);

        self.undo.push(undo_action);
        
        // After a user action is performed, clear redo
        self.redo.clear();
    }

    fn try_perform_action(&mut self, circuit: &mut Circuit, action: Action) {
        if let Some(undo_action) = action.try_perform(circuit) {
            self.push_undo(undo_action);
        }
    }

    fn leave_state(&mut self) {
        let undo_action = match self.state {
            State::Initial => None,
            State::Drawing { ref mut undo, .. } => {
                if undo.len() > 0 {
                    Some(Action::ReverseCompound(undo.clone()))
                } else {
                    None
                }
            },
            State::PlaceElement { .. } => None,
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
                    input::Keycode::Num1 => {
                        self.change_state(State::Initial);
                    }
                    input::Keycode::Num2 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Source,
                            rotation_cw: 0,
                        });
                    }
                    input::Keycode::Num3 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Sink,
                            rotation_cw: 0,
                        });
                    }
                    _ => {}
                }

                self.key_down_event(circuit, camera, keycode);
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
        _circuit: &mut Circuit,
        _camera: &Camera,
        _mouse_x: i32,
        _mouse_y: i32
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
                        let component = Element::Node
                            .new_component(grid_coords, 0);
                        let action = Action::PlaceComponent(component);
                        let undo_action = action.try_perform(circuit);

                        self.change_state(State::Drawing {
                            last_grid_coords: grid_coords,
                            axis_lock: None,
                            undo: undo_action.into_iter().collect()
                        });
                    }
                    input::MouseButton::Right => {
                        let action = Action::RemoveComponentAtPos(grid_coords);
                        self.try_perform_action(circuit, action);
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
            State::PlaceElement { element, rotation_cw } => {
                match button {
                    input::MouseButton::Left => {
                        // Use cursor pos as center if possible
                        let c = grid_coords - element.descr().size / 2;

                        let component = element.new_component(c, rotation_cw);
                        let action = Action::PlaceComponent(component);
                        self.try_perform_action(circuit, action);
                    }
                    input::MouseButton::Right => {
                        let action = Action::RemoveComponentAtPos(grid_coords);
                        self.try_perform_action(circuit, action);
                    }
                    _ =>  {}
                }
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _circuit: &mut Circuit,
        _camera: &Camera,
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
            State::PlaceElement { .. } => {}
        }
    }

    fn key_down_event(
        &mut self,
        circuit: &mut Circuit,
        _camera: &Camera,
        keycode: input::Keycode,
    ) {
        match self.state {
            State::Initial | State::PlaceElement { .. } => {
                match keycode {
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
                    _ => {}
                }
            }
            _ => {}
        }

        match self.state {
            State::PlaceElement { ref mut rotation_cw, .. } => {
                match keycode {
                    input::Keycode::R => {
                        *rotation_cw += 1
                    }
                    _ => {}
                }
            }
            _ => {}
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
                        circuit::Coords::new(self.grid_coords.x,
                                          last_grid_coords.y),
                    Some(Axis::Vertical) =>
                        circuit::Coords::new(last_grid_coords.x,
                                          self.grid_coords.y),
                    None =>
                        self.grid_coords 
                };

                if locked_coords != *last_grid_coords {
                    // We might have jumped more than one grid point.
                    // In this case, draw two lines to get there
                    // FIXME: Wrong lines being drawn
                    let min_x = cmp::min(locked_coords.x, last_grid_coords.x);
                    let max_x = cmp::max(locked_coords.x, last_grid_coords.x);
                    let min_y = cmp::min(locked_coords.y, last_grid_coords.y);
                    let max_y = cmp::max(locked_coords.y, last_grid_coords.y);

                    let line_v = (min_x..max_x+1).zip(once(min_y).cycle());
                    let line_h = once(max_x).cycle().zip(min_y..max_y+1);
                    let lines = line_v.chain(line_h);

                    let mut prev_c = None;
                    for (x, y) in lines {
                        let c = circuit::Coords::new(x, y);

                        if prev_c.is_none() || Some(c) != prev_c {
                            let component = Element::Node.new_component(c, 0);
                            let action = Action::PlaceComponent(component);
                            if let Some(u_action) = action.try_perform(circuit) {
                                undo.push(u_action);
                            }

                            if let Some(p) = prev_c {
                                let dir = Dir::from_coords(p, c);
                                let edge = circuit::Edge {
                                    layer: circuit::Layer::Ground
                                };

                                let action = Action::PlaceEdgeAtPos(p, dir, edge);
                                if let Some(u_action) = action.try_perform(circuit) {
                                    undo.push(u_action); 
                                }

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
            State::PlaceElement { .. } => {}
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        camera: &Camera,
        circuit: &Circuit,
        display: &Display
    ) -> GameResult<()> {
        // Draw current state 
        match self.state {
            State::PlaceElement { element, rotation_cw } => {
                // Use cursor pos as center if possible
                let c = self.grid_coords - element.descr().size / 2;

                let component =
                    element.new_component(c, rotation_cw);
                let action = Action::PlaceComponent(component.clone());

                let draw_mode =
                    if action.can_perform(circuit) {
                        display::DrawMode::Plan
                    } else {
                        display::DrawMode::Invalid
                    };

                display.draw_component(ctx, camera, &component, draw_mode)?; 
            },
            _ => {}
        }

        // Show grid coords that mouse is over
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

        // Some text for debugging
        let state_str = format!("{:?}", self.state);
        let state_text = graphics::Text::new(ctx, &state_str, &self.font)?;
        let state_text_pos = graphics::Point::new(
            10.0 + state_text.width() as f32 / 2.0, 10.0);
        state_text.draw(ctx, state_text_pos, 0.0)?;

        let coords_str = format!("({}, {})", self.grid_coords.x,
                                 self.grid_coords.y);
        let coords_text = graphics::Text::new(ctx, &coords_str, &self.font)?;
        let coords_text_pos = graphics::Point::new(
            10.0 + coords_text.width() as f32 / 2.0, 30.0);
        coords_text.draw(ctx, coords_text_pos, 0.0)?;

        Ok(())
    }
}
