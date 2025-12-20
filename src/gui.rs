// Contains stuff for sourcing images for fonts and rendering them
mod font;
// Contains the components of the GUI that the users use to paint to a canvas
mod draw_mode;

use font::TextmodeFont;

/// GUI for image based textmode. Powered by raylib
use raylib::prelude::*;

use crate::{
    gui::draw_mode::{CharsetPicker, ColorPicker, UserCanvas},
    model::CanvasBuilder,
    SadieError,
};

struct Attr {
    fg: Color,
    bg: Color,
}

impl Default for Attr {
    fn default() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

pub struct Env {
    rl: RaylibHandle,
    rt: RaylibThread,
    user_canvas: UserCanvas,
    charset_picker: CharsetPicker,
    color_picker: ColorPicker,
}

const DEFAULT_ID: u16 = 255;
impl Env {
    fn setup_canvas_texture(
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        charset: &TextmodeFont,
        columns: u16,
        rows: u16,
    ) -> Result<RenderTexture2D, SadieError> {
        let (char_width, char_height) = charset.quad_dimensions();
        let canvas_texture = rl
            .load_render_texture(rt, columns as u32 * char_width, rows as u32 * char_height)
            .map_err(SadieError::Raylib)?;

        Ok(canvas_texture)
    }

    pub fn new() -> Result<Self, SadieError> {
        let (mut rl, rt) = init().title("Sadie").build();

        let charset = TextmodeFont::load_charset(&mut rl, &rt, "gloop_8x8.png", 16, 16)?;
        let (columns, rows) = (8, 8);
        let canvas_texture = Self::setup_canvas_texture(&mut rl, &rt, &charset, columns, rows)?;

        let uc = CanvasBuilder::init(charset.clone())
            .size(16, 14)
            .build();

        let user_canvas = UserCanvas::new(uc);


        Ok(Self {
            rl,
            rt,
            user_canvas
        })
    }

    pub fn run(&mut self) -> Result<(), SadieError> {
        let thread = &self.rt;
        let src = &self.user_canvas.charset().source;
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(thread);
            d.clear_background(Color::RED);

            d.draw_texture_mode(thread, &mut self.canvas_texture, |mut rd| {
                for (x, y, rect, attr) in self.user_canvas.cells() {
                    rd.draw_texture_rec(
                        src,
                        rect,
                        Vector2 {
                            x: x as f32 * rect.width,
                            y: y as f32 * rect.height,
                        },
                        Color::WHITE,
                    );
                }
            });

            d.draw_texture(&self.canvas_texture, 0, 0, Color::WHITE);
        }

        Ok(())
    }
}
