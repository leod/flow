use std::cmp;
use std::iter::{once, Iterator};

use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics::{self, Drawable};
use sdl2::keyboard;

use types::{Dir, Axis};
use input::{self, Input};
use camera::Camera;
use circuit::{self, ChipId, ChipDb, Circuit, Action, SwitchType, Element};
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
    pub font: graphics::Font,
    
    cur_chip_id: Option<ChipId>,

    state: State,
    undo: Vec<(Option<ChipId>, Action)>,
    redo: Vec<(Option<ChipId>, Action)>,

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
            cur_chip_id: None,
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

    pub fn cur_chip_id(&self) -> &Option<ChipId> {
        &self.cur_chip_id
    }

    fn change_state(&mut self, new_state: State) {
        self.leave_state();
        self.state = new_state;
    }

    fn push_undo(&mut self, undo_action: Action) {
        //println!("undo: {:?}", undo_action);

        self.undo.push((self.cur_chip_id, undo_action));
        
        // After a user action is performed, clear redo
        self.redo.clear();
    }

    fn circuit_mut<'a>(
        &self,
        chip_id: &Option<ChipId>,
        circuit: &'a mut Circuit,
        chip_db: &'a mut ChipDb
    ) -> &'a mut Circuit {
        match chip_id {
            &Some(ref chip_id) =>
                chip_db.get_circuit_mut(chip_id).unwrap(),
            &None =>
                circuit
        }
    }

    fn circuit<'a>(
        &self,
        chip_id: &Option<ChipId>,
        circuit: &'a Circuit,
        chip_db: &'a ChipDb
    ) -> &'a Circuit {
        match chip_id {
            &Some(ref chip_id) =>
                chip_db.get_circuit(chip_id).unwrap(),
            &None =>
                circuit
        }
    }

    fn cur_circuit_mut<'a>(
        &self,
        circuit: &'a mut Circuit,
        chip_db: &'a mut ChipDb
    ) -> &'a mut Circuit {
        self.circuit_mut(&self.cur_chip_id, circuit, chip_db)
    }

    pub fn cur_circuit<'a>(
        &self,
        circuit: &'a Circuit,
        chip_db: &'a ChipDb
    ) -> &'a Circuit {
        self.circuit(&self.cur_chip_id, circuit, chip_db)
    }

    pub fn switch_chip(&mut self, chip_id: &Option<ChipId>) {
        self.change_state(State::Initial);
        self.cur_chip_id = chip_id.clone();
    }

    fn keycode_to_chip_id(&self, keycode: input::Keycode) -> Option<ChipId> {
        match keycode {
            input::Keycode::F2 => Some(2),
            input::Keycode::F3 => Some(3),
            input::Keycode::F4 => Some(4),
            input::Keycode::F5 => Some(5),
            input::Keycode::F6 => Some(6),
            input::Keycode::F7 => Some(7),
            input::Keycode::F8 => Some(8),
            input::Keycode::F9 => Some(9),
            input::Keycode::F10 => Some(10),
            _ => None
        }
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
        chip_db: &mut ChipDb,
        camera: &Camera,
        input: &Input
    ) {
        match input {
            &Input::MouseMotion { state: _, x, y, xrel: _, yrel: _ } => {
                self.mouse_x = x;
                self.mouse_y = y;

                self.mouse_motion_event(circuit, chip_db, camera, x, y);
            }
            &Input::MouseButtonDown { button, x, y } => {
                self.mouse_button_down_event(circuit, chip_db, camera, button, x, y);
            }
            &Input::MouseButtonUp { button, x, y } => {
                self.mouse_button_up_event(circuit, chip_db, camera, button, x, y);
            }
            &Input::KeyDown { keycode, keymod, repeat: _ } => {
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
                    input::Keycode::Num4 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Switch(SwitchType::On),
                            rotation_cw: 0,
                        });
                    }
                    input::Keycode::Num5 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Switch(SwitchType::Off),
                            rotation_cw: 0,
                        });
                    }
                    input::Keycode::Num6 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Bridge,
                            rotation_cw: 0,
                        });
                    }
                    input::Keycode::Num7 => {
                        self.change_state(State::PlaceElement {
                            element: Element::Power,
                            rotation_cw: 0,
                        });
                    }
                    input::Keycode::F1 => {
                        self.switch_chip(&None);
                    }
                    keycode => {
                        if let Some(chip_id) = self.keycode_to_chip_id(keycode) {
                            if keymod.contains(keyboard::LSHIFTMOD) {
                                self.switch_chip(&Some(chip_id));
                            } else {
                                let chip_descr = chip_db.get_descr(&chip_id).unwrap();
                                self.change_state(State::PlaceElement {
                                    element: Element::Chip(chip_id, chip_descr.clone()),
                                    rotation_cw: 0, 
                                });
                            }
                        }
                    }
                }

                self.key_down_event(circuit, chip_db, camera, keycode);
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
        _chip_db: &mut ChipDb,
        _camera: &Camera,
        _mouse_x: i32,
        _mouse_y: i32
    ) {
    }

    fn mouse_button_down_event(
        &mut self,
        circuit: &mut Circuit,
        chip_db: &mut ChipDb,
        camera: &Camera,
        button: input::MouseButton,
        x: i32,
        y: i32
    ) {
        let cur_circuit = self.cur_circuit_mut(circuit, chip_db);
        let grid_coords = screen_to_grid_coords(camera, x, y);

        match self.state.clone() {
            State::Initial => {
                match button {
                    input::MouseButton::Left => {
                        let component = Element::Node
                            .new_component(grid_coords, 0);
                        let action = Action::PlaceComponent(component);
                        let undo_action = action.try_perform(cur_circuit);

                        self.change_state(State::Drawing {
                            last_grid_coords: grid_coords,
                            axis_lock: None,
                            undo: undo_action.into_iter().collect()
                        });
                    }
                    input::MouseButton::Right => {
                        let action = Action::RemoveComponentAtPos(grid_coords);
                        self.try_perform_action(cur_circuit, action);
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
            State::PlaceElement { ref element, rotation_cw } => {
                match button {
                    input::MouseButton::Left => {
                        // Use cursor pos as center if possible
                        let c = grid_coords - element.descr().size / 2;

                        let component = element.new_component(c, rotation_cw);
                        let action = Action::PlaceComponent(component);
                        self.try_perform_action(cur_circuit, action);
                    }
                    input::MouseButton::Right => {
                        let action = Action::RemoveComponentAtPos(grid_coords);
                        self.try_perform_action(cur_circuit, action);
                    }
                    _ =>  {}
                }
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _circuit: &mut Circuit,
        _chip_db: &mut ChipDb,
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
        chip_db: &mut ChipDb,
        _camera: &Camera,
        keycode: input::Keycode,
    ) {
        match self.state {
            State::Initial | State::PlaceElement { .. } => {
                match keycode {
                    input::Keycode::Z => {
                        if self.hold_control == true {
                            if let Some((chip_id, undo_action)) = self.undo.pop() {
                                let action_circuit = self.circuit_mut(&chip_id, circuit, chip_db);
                                let redo_action = undo_action.perform(action_circuit);
                                self.redo.push((chip_id.clone(), redo_action));
                                self.switch_chip(&chip_id);
                            }
                        }
                    }
                    input::Keycode::Y => {
                        if self.hold_control == true {
                            if let Some((chip_id, redo_action)) = self.redo.pop() {
                                let action_circuit = self.circuit_mut(&chip_id, circuit, chip_db);
                                let undo_action = redo_action.perform(action_circuit);
                                self.undo.push((chip_id.clone(), undo_action));
                                self.switch_chip(&chip_id);
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
        chip_db: &mut ChipDb,
        camera: &Camera,
        _dt_s: f32
    ) {
        let cur_circuit = self.cur_circuit_mut(circuit, chip_db);
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
                            if let Some(u_action) = action.try_perform(cur_circuit) {
                                undo.push(u_action);
                            }

                            if let Some(p) = prev_c {
                                let dir = Dir::from_coords(p, c);
                                let edge = circuit::Edge {};

                                let action = Action::PlaceEdgeAtPos(p, dir, Some(edge));
                                if let Some(u_action) = action.try_perform(cur_circuit) {
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
        circuit: &Circuit,
        chip_db: &ChipDb,
        camera: &Camera,
        display: &Display
    ) -> GameResult<()> {
        let cur_circuit = self.cur_circuit(circuit, chip_db);

        // Draw current state 
        match self.state.clone() {
            State::PlaceElement { ref element, rotation_cw } => {
                // Use cursor pos as center if possible
                let c = self.grid_coords - element.descr().size / 2;

                let component =
                    element.new_component(c, rotation_cw);
                let action = Action::PlaceComponent(component.clone());

                let draw_mode =
                    if action.can_perform(cur_circuit) {
                        display::DrawMode::Plan
                    } else {
                        display::DrawMode::Invalid
                    };

                display.draw_component(ctx, &self.font, camera, &component, draw_mode)?; 
            },
            _ => {}
        }

        // Show grid coords that mouse is over
        let grid_coords_t = camera.transform(
            self.grid_coords.cast() * display::EDGE_LENGTH);

        let r = graphics::Rect {
            x: grid_coords_t.x,
            y: grid_coords_t.y,
            w: camera.transform_distance(display::EDGE_LENGTH / 6.0),
            h: camera.transform_distance(display::EDGE_LENGTH / 6.0)
        };

        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;

        // Some text for debugging
        /*let state_str = format!("{:?}", self.state);
        let state_text = graphics::Text::new(ctx, &state_str, &self.font)?;
        let state_text_pos = graphics::Point::new(
            10.0 + state_text.width() as f32 / 2.0, 10.0);
        state_text.draw(ctx, state_text_pos, 0.0)?;*/

        /*let coords_str = format!("({}, {})", self.grid_coords.x,
                                 self.grid_coords.y);
        let coords_text = graphics::Text::new(ctx, &coords_str, &self.font)?;
        let coords_text_pos = graphics::Point::new(
            10.0 + coords_text.width() as f32 / 2.0, 30.0);
        coords_text.draw(ctx, coords_text_pos, 0.0)?;*/

        let chip_str = match self.cur_chip_id {
            Some(ref cur_chip_id) => format!("Chip {:?}", cur_chip_id),
            None => format!("Main circuit")
        };
        let chip_text = graphics::Text::new(ctx, &chip_str, &self.font)?;
        let chip_text_pos = graphics::Point::new(
            10.0 + chip_text.width() as f32 / 2.0, 30.0);
        chip_text.draw(ctx, chip_text_pos, 0.0)?;

        Ok(())
    }
}
