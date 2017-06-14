use ggez::{GameResult, Context};
use ggez::graphics;
use input::{self, Input};

pub struct Hud {
    mx: i32,
    my: i32,
    font: graphics::Font,
}

impl Hud {
    pub fn new(ctx: &mut Context) -> GameResult<Hud> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        let h = Hud {
            mx: ctx.conf.window_width as i32 / 2,
            my: ctx.conf.window_height as i32 / 2,
            font: font
        };
        Ok(h)
    }

	pub fn input_event(&mut self, input: &Input) {
        match input {
            &Input::MouseMotion { state: _, x, y, xrel: _, yrel: _ } => {
                self.mx = x;
                self.my = y;
            }
            _ => {}
        }
    }		

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let r = graphics::Rect {
            x: self.mx as f32,
            y: self.my as f32,
            w: 3.0,
            h: 3.0
        };
        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;

        Ok(())
    }
}
