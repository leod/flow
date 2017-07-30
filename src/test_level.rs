use rand::{self, Rng};

use circuit;
use flow;
use level::{Level, Outcome, LevelImpl};

pub struct TestLevel {
    seq: Vec<bool>,
    max_epochs: usize,
    reverse: bool,
    written: usize,
    read: usize,
    epochs: usize,
}

impl LevelImpl for TestLevel {
    fn time_step(&mut self, state: &mut flow::State) -> Option<Outcome> {
        if self.written < self.seq.len() {
            state.flow.node_mut(state.input_cells[0]).enabled = true;
            state.flow.node_mut(state.input_cells[1]).enabled = self.seq
                [self.written];
            println!("write {}", self.seq[self.written]);
            self.written += 1;
        } else {
            state.flow.node_mut(state.input_cells[0]).enabled = false;
            state.flow.node_mut(state.input_cells[1]).enabled = false;
        }

        if state.flow.node(state.output_cells[0]).in_flow > 0.001 {
            let output = state.flow.node(state.output_cells[1]).in_flow > 0.001;
            println!("read {}", output);

            let idx = if self.reverse {
                self.seq.len() - (self.read + 1)
            } else {
                self.read
            };

            if output != self.seq[idx] {
                Some(Outcome::Failure)
            } else {
                self.read += 1;
                if self.read == self.seq.len() {
                    self.read = 0;
                    self.epochs += 1;
                    if self.epochs == self.max_epochs {
                        Some(Outcome::Success)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

pub fn test_level() -> Level {
    Level {
        input_size: 2,
        input_pos: circuit::Coords::new(0, 0),
        output_size: 2,
        output_pos: circuit::Coords::new(20, 0),
        create_impl: Box::new(|| {
            let seq = (1..4).map(|_| rand::thread_rng().gen()).collect();
            let state = TestLevel {
                seq,
                max_epochs: 1,
                reverse: true,
                written: 0,
                read: 0,
                epochs: 0,
            };
            Box::new(state)
        }),
    }
}
