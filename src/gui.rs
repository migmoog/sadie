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
use crate::gui::draw_mode::{Draw, Palette, Attr};

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

        let charset_picker = CharsetPicker::new(charset.clone(), &mut rl, &rt);
        let uc = CanvasBuilder::init(charset.clone())
            .size(8, 8)
            .build::<Attr>();
        let user_canvas = UserCanvas::new(&mut rl, &rt, uc);

        let cp_target = rl.load_render_texture(&rt, 8 * 16, 8 * 16).unwrap();
        let color_picker = ColorPicker::new(Palette::PICO8.into(), cp_target);

        Ok(Self {
            rl,
            rt,
            user_canvas,
            color_picker,
            charset_picker
        })
    }

    pub fn run(&mut self) -> Result<(), SadieError> {
        let thread = &self.rt;
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(thread);
            d.clear_background(Color::RAYWHITE);

            self.charset_picker.draw(&mut d, thread);
            self.color_picker.draw(&mut d, thread);
            self.user_canvas.draw(&mut d, thread);
        }

        Ok(())
    }
}
