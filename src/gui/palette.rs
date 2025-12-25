use crate::{gui::GuiCharset, model::{CharID, Charset}};
use euclid::default::Size2D;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Palette(Vec<Color>, Size2D<u16>);

impl Charset for Palette {
    fn len(&self) -> u16 {
        self.0.len() as u16
    }

    type Item = Color;
    fn get_char(&self, id: CharID) -> Self::Item {
        self.0[id as usize]
    }
}

impl Palette {
    pub const PICO8: [Color; 16] = [
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }, // Black
        Color {
            r: 29,
            g: 43,
            b: 83,
            a: 255,
        }, // Dark Blue
        Color {
            r: 126,
            g: 37,
            b: 83,
            a: 255,
        }, // Dark Purple
        Color {
            r: 0,
            g: 135,
            b: 81,
            a: 255,
        }, // Dark Green
        Color {
            r: 171,
            g: 82,
            b: 54,
            a: 255,
        }, // Brown
        Color {
            r: 95,
            g: 87,
            b: 79,
            a: 255,
        }, // Dark Gray
        Color {
            r: 194,
            g: 195,
            b: 199,
            a: 255,
        }, // Light Gray
        Color {
            r: 255,
            g: 241,
            b: 232,
            a: 255,
        }, // White
        Color {
            r: 255,
            g: 0,
            b: 77,
            a: 255,
        }, // Red
        Color {
            r: 255,
            g: 163,
            b: 0,
            a: 255,
        }, // Orange
        Color {
            r: 255,
            g: 236,
            b: 39,
            a: 255,
        }, // Yellow
        Color {
            r: 0,
            g: 228,
            b: 54,
            a: 255,
        }, // Green
        Color {
            r: 41,
            g: 173,
            b: 255,
            a: 255,
        }, // Blue
        Color {
            r: 131,
            g: 118,
            b: 156,
            a: 255,
        }, // Indigo
        Color {
            r: 255,
            g: 119,
            b: 168,
            a: 255,
        }, // Pink
        Color {
            r: 255,
            g: 204,
            b: 170,
            a: 255,
        }, // Peach
    ];
}

impl<const N: usize> From<[Color; N]> for Palette {
    fn from(colors: [Color; N]) -> Self {
        Self(colors.into(), (8, 8).into())
    }
}

impl GuiCharset for Palette {
    fn get_char_size(&self) -> Size2D<u16> {
        self.1
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self(Self::PICO8.into(), (8, 8).into())
    }
}
