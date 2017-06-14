use cgmath::Vector2;

use ggez::{GameResult, Context};
use ggez::graphics;

use input::{self, Input};
use camera::Camera;
use grid;

pub struct Hud {
    mouse_x: i32,
    mouse_y: i32,
    grid_coords: grid::Coords,

    font: graphics::Font,
}

impl Hud {
    pub fn new(ctx: &mut Context) -> GameResult<Hud> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;

        let h = Hud {
            mouse_x: ctx.conf.window_width as i32 / 2,
            mouse_y: ctx.conf.window_height as i32 / 2,
            grid_coords: grid::Coords::new(0, 0),
            font: font
        };
        Ok(h)
    }

	pub fn input_event(&mut self, input: &Input) {
        match input {
            &Input::MouseMotion { state: _, x, y, xrel: _, yrel: _ } => {
                self.mouse_x = x;
                self.mouse_y = y;
            }
            _ => {}
        }
    }		

    pub fn update(&mut self, _ctx: &mut Context, camera: &Camera, dt_s: f32) {
        let mouse_p_t = Vector2::new(self.mouse_x as f32, self.mouse_y as f32);
        let mouse_p = camera.untransform(mouse_p_t);
        let g_x = mouse_p.x.round() as isize;
        let g_y = mouse_p.y.round() as isize;
        
        self.grid_coords = grid::Coords::new(g_x, g_y);
    }

    pub fn draw(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult<()> {
        let grid_coords_t = camera.transform(self.grid_coords.cast());

        let r = graphics::Rect {
            x: grid_coords_t.x,
            y: grid_coords_t.y,
            w: 3.0,
            h: 3.0
        };

        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, r)?;

        Ok(())
    }
}
