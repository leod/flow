use ggez::{GameResult, Context};
use ggez::graphics;
use input::{self, Input};

pub struct Hud {
    font: graphics::Font,
}

impl Hud {
    pub fn new(ctx: &mut Context) -> GameResult<Hud> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        let h = Hud {
            font: font
        };
        Ok(h)
    }

	pub fn input_event(&mut self, input: &Input) {

    }		
}
