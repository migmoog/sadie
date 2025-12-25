// Contains stuff for sourcing images for fonts and rendering them
mod font;
mod palette;
// Contains the components of the GUI that the users use to paint to a canvas
mod draw_mode;

use std::collections::HashMap;

use euclid::default::Size2D;
use font::TextmodeFont;

/// GUI for image based textmode. Powered by raylib
use raylib::prelude::*;

use crate::gui::draw_mode::{GuiComponent, Palette};
use crate::model::{CanvasBuilder, CanvasPos, Charset};
use crate::SadieError;

pub trait GuiCharset: Charset {
    fn get_char_size(&self) -> Size2D<u16>;
}

pub struct Client {
    rl: RaylibHandle,
    rt: RaylibThread,

    ui_components: HashMap<CanvasPos, GuiComponent>,
}

impl Client {
    pub fn new() -> Result<Self, SadieError> {
        let (mut rl, rt) = init().title("Sadie").build();

        let charset = TextmodeFont::load_charset(&mut rl, &rt, "gloop_8x8.png", 16, 16)?;
        let palette = Palette::default();

        let ui_components = HashMap::from([
            (
                (0, 0).into(),
                GuiComponent::make_user_canvas(
                    &mut rl,
                    &rt,
                    CanvasBuilder::init(charset.clone())
                        .cursor_position(0, 0)
                        .size(16, 14)
                        .build(),
                ),
            ),
            (
                (0, 200).into(),
                GuiComponent::make_charset_picker(charset.clone(), &mut rl, &rt),
            ),
            (
                (0, 150).into(),
                GuiComponent::make_color_picker(palette, &mut rl, &rt),
            ),
        ]);

        Ok(Self {
            rl,
            rt,
            ui_components,
        })
    }

    pub fn run(&mut self) -> Result<(), SadieError> {
        let thread = &self.rt;
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(thread);
            d.clear_background(Color::RAYWHITE);

            for (&p, comp) in self.ui_components.iter_mut() {
                comp.draw(p, &mut d, thread);
            }
        }

        Ok(())
    }
}
