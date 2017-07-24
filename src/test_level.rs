use circuit;
use flow;
use level::{Level, Outcome, LevelImpl};

pub struct TestLevel {

}

impl LevelImpl for TestLevel {
    fn time_step(&mut self, flow: &flow::State) -> Option<Outcome> {
        None
    }
}

pub fn test_level() -> Level {
    Level {
        input_size: 3,
        input_pos: circuit::Coords::new(0, 0),
        output_size: 2,
        output_pos: circuit::Coords::new(10, 0),
        create_impl: Box::new(|| Box::new(TestLevel{}))
    }
}
