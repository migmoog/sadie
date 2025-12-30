// Contains stuff for sourcing images for fonts and rendering them
mod font;
mod palette;
// Contains the components of the GUI that the users use to paint to a canvas
// mod user_interface;

// More agnostic way of describing how to draw canvases
mod gallery;

use euclid::default::Size2D;
use font::TextmodeFont;
use palette::Palette;

/// GUI for image based textmode. Powered by raylib
use raylib::prelude::*;

use crate::core::canvas::CanvasBuilder;
use crate::core::Charset;
use crate::gui::gallery::{CellColors, GuiGallery};
use crate::{SadieContext, SadieError};

pub trait GuiCharset: Charset {
    /// Return the size of a character in the set in pixels
    fn get_char_size(&self) -> Size2D<u16>;

    /// Creates a render texture to be drawn to a canvas
    fn make_canvas_rtex(
        &self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        canvas_size: Size2D<u32>,
    ) -> Result<RenderTexture2D, SadieError> {
        let size = self.get_char_size();
        rl.load_render_texture(
            rt,
            size.width as u32 * canvas_size.width,
            size.height as u32 * canvas_size.height,
        )
        .map_err(SadieError::Raylib)
    }
}

pub struct RaylibContext {
    rl: RaylibHandle,
    rt: RaylibThread,
    gallery: GuiGallery,
}

impl Default for RaylibContext {
    fn default() -> Self {
        let (mut rl, rt) = raylib::init().size(800, 800).title("Sadie").build();

        rl.set_exit_key(None);

        let charset = TextmodeFont::load_charset(&mut rl, &rt, "gloop_8x8.png", 16, 16).unwrap();
        let user_canvas = CanvasBuilder::init(charset.clone())
            .cursor_position(0, 0)
            .size((12, 8).into())
            .default_cells(|_, c| {
                // NOTE: might have some problems with two's complement. This function could really
                // use some reworking
                let id: i32 = rl.get_random_value(0..{ c.len() - 1 }.into());
                (id as u16, CellColors::default())
            })
            .build();
        let mut gallery = GuiGallery::new();
        if gallery.add_colored_font(&mut rl, &rt, user_canvas).is_err() {
            println!("Couldn't add user canvas");
        }

        let charset_picker = CanvasBuilder::init(charset.clone())
            .cursor_position(0, 0)
            .char_cascade()
            .build();

        if gallery
            .add_font_only(&mut rl, &rt, charset_picker)
            .map(|e| e.and_modify(|f| f.position = (0, 200).into()))
            .is_err()
        {
            println!("Couldn't add charset picker");
        }

        let color_picker = CanvasBuilder::init(Palette::default())
            .cursor_position(0, 0)
            .cursor_position(1, 0)
            .char_cascade()
            .build();

        if gallery
            .add_color_squares(&mut rl, &rt, color_picker)
            .map(|e| e.and_modify(|f| f.position = (0, 300).into()))
            .is_err()
        {
            println!("Couldn't add color picker");
        }

        Self { rl, rt, gallery }
    }
}

impl SadieContext for RaylibContext {
    fn check_input(&mut self) -> Option<char> {
        self.rl.get_char_pressed()
    }

    fn is_alive(&self) -> bool {
        !self.rl.window_should_close()
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.rt);
        d.clear_background(Color::WHITE);

        self.gallery.draw(&mut d, &self.rt);
    }

    fn apply_actions(&mut self, action: crate::core::actions::Action) -> Result<(), SadieError> {
        todo!()
    }
}
