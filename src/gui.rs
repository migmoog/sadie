/// GUI for image based textmode. Powered by raylib
use raylib::prelude::*;
use std::{char, collections::HashMap};

use crate::{
    model::{CanvasModel, CharID, Charset},
    SadieError,
};

struct CellAttr {
    fg: Color,
    bg: Color,
}

impl Default for CellAttr {
    fn default() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

struct TextmodeFont {
    source: Texture2D,
    char_quads: HashMap<CharID, Rectangle>,
}

impl Charset for TextmodeFont {
    type Item = Rectangle;
    fn get_char(&self, id: CharID) -> Self::Item {
        *self
            .char_quads
            .get(&id)
            .expect("rectangle for this id not found")
    }
}

impl TextmodeFont {
    fn make_char_quads(
        texture_width: i32,
        texture_height: i32,
        columns: u16,
        rows: u16,
    ) -> HashMap<u16, Rectangle> {
        let (cell_width, cell_height) =
            (
                { texture_width / columns as i32 } as f32,
                { texture_height / rows as i32 } as f32,
            );
        (0..{ columns * rows })
            .map(|i| {
                let (x, y) = ({ i % columns } as f32, { i / columns } as f32);

                (
                    i,
                    Rectangle {
                        x: x * cell_width,
                        y: y * cell_height,
                        width: cell_width,
                        height: cell_height,
                    },
                )
            })
            .collect()
    }

    fn load_charset(
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        filename: &str,
        columns: u16,
        rows: u16,
    ) -> Result<Self, SadieError> {
        let source = rl.load_texture(rt, filename).map_err(SadieError::Raylib)?;
        // should only have the colors black and white
        let palette = source
            .load_image()
            .map_err(SadieError::Raylib)?
            .extract_palette(3);
        if !(palette.len() == 2
            && [Color::BLACK, Color::WHITE]
                .into_iter()
                .all(|item| palette.contains(&item)))
        {
            return Err(SadieError::NotBlackAndWhite {
                fontname: filename.into(),
                palette,
            });
        }

        let char_quads = Self::make_char_quads(source.width(), source.height(), columns, rows);

        Ok(Self { source, char_quads })
    }

    /// Helper for creating canvases
    fn quad_dimensions(&self) -> (u32, u32) {
        let (_, q) = self
            .char_quads
            .iter()
            .next()
            .expect("textmode font has no quads");

        (q.width as u32, q.height as u32)
    }
}

#[cfg(test)]
mod textmode_font_test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn making_char_quads() {
        let qs = TextmodeFont::make_char_quads(128, 128, 16, 16);
        assert_eq!(
            qs.get(&0),
            Some(&Rectangle {
                x: 0.,
                y: 0.,
                width: 8.,
                height: 8.
            })
        );

        assert_eq!(
            qs.get(&255),
            Some(&Rectangle {
                x: 120.0,
                y: 120.0,
                width: 8.,
                height: 8.
            })
        );
    }
}

pub struct Env {
    rl: RaylibHandle,
    rt: RaylibThread,
    user_canvas: CanvasModel<TextmodeFont, CellAttr>,
    canvas_texture: RenderTexture2D,
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
        let mut user_canvas = CanvasModel::new(columns, rows, charset);
        let (id, _) = user_canvas.get_mut(3, 3);
        *id = DEFAULT_ID;

        Ok(Self {
            rl,
            rt,
            user_canvas,
            canvas_texture,
        })
    }

    pub fn run(&mut self) -> Result<(), SadieError> {
        let thread = &self.rt;
        let src = &self.user_canvas.charset().source;
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(thread);
            d.clear_background(Color::RED);

            d.draw_texture(src, 0, 120, Color::WHITE);
            d.draw_texture_rec(
                src,
                Rectangle {
                    x: 8.0,
                    y: 8.0,
                    width: 8.,
                    height: 8.,
                },
                Vector2 { x: 250.0, y: 120.0 },
                Color::WHITE,
            );

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
