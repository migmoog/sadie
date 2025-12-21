use euclid::default::Size2D;
use crate::{
    gui::font::TextmodeFont,
    model::{Canvas, CanvasBuilder, Cell, CharID, Charset},
};
use raylib::prelude::*;

pub struct Attr {
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

pub trait Draw {
    fn draw<Rd>(&mut self, rl: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt;
}

struct GuiCanvas<C, A = ()> {
    canvas: Canvas<C, A>,
    target: RenderTexture2D,
}

impl<C, A> AsRef<raylib::ffi::Texture> for GuiCanvas<C, A> {
    fn as_ref(&self) -> &raylib::ffi::Texture {
        self.target.as_ref()
    }
}

impl<A> GuiCanvas<TextmodeFont, A> {
    fn with_target(
        canvas: Canvas<TextmodeFont, A>,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) -> Self {
        let (w, h) = canvas.charset().quad_dimensions();
        let s = canvas.size();

        let target = rl
            .load_render_texture(&rt, w * s.width as u32, h * s.height as u32)
            .expect("couldn't make render texture");

        Self { canvas, target }
    }
}

// welcome to GENERICS HELL 😈
impl<T, C, A> GuiCanvas<C, A>
where
    C: Charset<Item = T>,
{
    fn begin_canvas_mode<'a, Rd>(
        &'a mut self,
        rl: &'a mut Rd,
        rt: &'a RaylibThread,
    ) -> (impl Iterator<Item = Cell<T, A>>, RaylibTextureMode<'a, Rd>)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        let d = rl.begin_texture_mode(rt, &mut self.target);
        (self.canvas.cells(), d)
    }
}

#[derive(Clone)]
pub struct Palette(Vec<Color>);

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
        Color { r: 0, g: 0, b: 0, a: 255 },           // Black
        Color { r: 29, g: 43, b: 83, a: 255 },        // Dark Blue
        Color { r: 126, g: 37, b: 83, a: 255 },       // Dark Purple
        Color { r: 0, g: 135, b: 81, a: 255 },        // Dark Green
        Color { r: 171, g: 82, b: 54, a: 255 },       // Brown
        Color { r: 95, g: 87, b: 79, a: 255 },        // Dark Gray
        Color { r: 194, g: 195, b: 199, a: 255 },     // Light Gray
        Color { r: 255, g: 241, b: 232, a: 255 },     // White
        Color { r: 255, g: 0, b: 77, a: 255 },        // Red
        Color { r: 255, g: 163, b: 0, a: 255 },       // Orange
        Color { r: 255, g: 236, b: 39, a: 255 },      // Yellow
        Color { r: 0, g: 228, b: 54, a: 255 },        // Green
        Color { r: 41, g: 173, b: 255, a: 255 },      // Blue
        Color { r: 131, g: 118, b: 156, a: 255 },     // Indigo
        Color { r: 255, g: 119, b: 168, a: 255 },     // Pink
        Color { r: 255, g: 204, b: 170, a: 255 },     // Peach
    ];
}

impl<const N: usize> From<[Color;N]> for Palette {
    fn from(colors: [Color;N]) -> Self {
        Self(colors.into())
    }
}

pub struct CharsetPicker(GuiCanvas<TextmodeFont>);

impl CharsetPicker {
    pub fn new(tf: TextmodeFont, rl: &mut RaylibHandle, rt: &RaylibThread) -> Self {
        let l = tf.len();
        let mut canvas = CanvasBuilder::init(tf)
            .size(l, 1)
            .cursor_position(0, 0)
            .build_no_attrs();

        for (i, (id, _)) in canvas.iter_mut().enumerate() {
            *id = i as CharID;
        }

        Self(GuiCanvas::with_target(canvas, rl, rt))
    }
}

impl Draw for CharsetPicker {
    fn draw<Rd>(&mut self, rl: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        {
            let src = self.0.canvas.charset().source.clone();
            let (cells, mut rd) = self.0.begin_canvas_mode(rl, rt);

            for (x, y, r, _) in cells {
                rd.draw_texture_rec(
                    &src,
                    r,
                    Vector2 {
                        x: x as f32 * r.width,
                        y: y as f32 * r.height,
                    },
                    Color::WHITE,
                );
            }
        }

        rl.draw_texture(&self.0, 0, 120, Color::WHITE);
    }
}

pub struct ColorPicker(GuiCanvas<Palette>);

impl ColorPicker {
    pub fn new(p: Palette, target: RenderTexture2D) -> Self {
        let l = p.len();
        let mut canvas = CanvasBuilder::init(p)
            .size(l, 1)
            .cursor_position(0, 0)
            .build_no_attrs();

        for (i, (id, _)) in canvas.iter_mut().enumerate() {
            *id = i as CharID;
        }

        Self(GuiCanvas { canvas, target })
    }
}

impl Draw for ColorPicker {
    fn draw<Rd>(&mut self, rl: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        {
            let (cells, mut rd) = self.0.begin_canvas_mode(rl, rt);
            let (w, h) = (8, 8);
            for (x, y, color, _) in cells {
                rd.draw_rectangle(x as i32 * h, y as i32 * w, w, h, color);
            }
        }

        rl.draw_texture(&self.0, 0, 300, Color::WHITE);
    }
}

pub struct UserCanvas(GuiCanvas<TextmodeFont, Attr>);

impl UserCanvas {
    pub fn new(rl: &mut RaylibHandle, rt: &RaylibThread, canvas: Canvas<TextmodeFont, Attr>) -> Self {
        Self(GuiCanvas::with_target(canvas, rl, rt))
    }
}

impl Draw for UserCanvas {
    fn draw<Rd>(&mut self, rl: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        {
            let src = self.0.canvas.charset().source.clone();
            let (cells, mut rd) = self.0.begin_canvas_mode(rl, rt);
            for (x, y, r, a) in cells {
                rd.draw_texture_rec(
                    &src,
                    r,
                    Vector2 {
                        x: x as f32 * r.width,
                        y: y as f32 * r.height,
                    },
                    Color::WHITE,
                );
            }
        }
        rl.draw_texture(&self.0, 0, 0, Color::WHITE);
    }
}
