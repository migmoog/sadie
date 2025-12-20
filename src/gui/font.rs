use std::{collections::HashMap, ops::Deref, rc::Rc};
use raylib::prelude::*;

use crate::{SadieError, model::{CharID, Charset}};

#[cfg(test)]
mod textmode_font_test {
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

/// Reference counted source of a font
#[derive(Clone)]
pub struct TextmodeFontSource(Rc<Texture2D>);
impl TextmodeFontSource {
    fn new(texture: Texture2D) -> Self {
        Self(Rc::new(texture))
    }
}

impl AsRef<raylib::ffi::Texture2D> for TextmodeFontSource {
    fn as_ref(&self) -> &raylib::ffi::Texture2D {
        self.0.as_ref()
    }
}

impl Deref for TextmodeFontSource {
    type Target = Rc<Texture2D>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct TextmodeFont {
    pub source: TextmodeFontSource,
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

    fn len(&self) -> u16 {
        self.char_quads.len() as u16
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

    pub fn load_charset(
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        filename: &str,
        columns: u16,
        rows: u16,
    ) -> Result<Self, SadieError> {
        let source = TextmodeFontSource::new(rl.load_texture(rt, filename).map_err(SadieError::Raylib)?);
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
    pub fn quad_dimensions(&self) -> (u32, u32) {
        let (_, q) = self
            .char_quads
            .iter()
            .next()
            .expect("textmode font has no quads");

        (q.width as u32, q.height as u32)
    }
}
