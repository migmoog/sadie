use crate::{
    gui::font::TextmodeFont,
    model::{Canvas, CanvasBuilder, Cell, CharID, Charset},
};
use raylib::prelude::*;

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
        tf: TextmodeFont,
        canvas: Canvas<TextmodeFont, A>,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) -> Self {
        let (w, h) = tf.quad_dimensions();
        let s = canvas.size();

        let target = rl.load_render_texture(&rt, w * s.width as u32, h * s.height as u32).expect("couldn't make render texture");

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

pub struct CharsetPicker(GuiCanvas<TextmodeFont>);

impl CharsetPicker {
    pub fn new(tf: TextmodeFont, target: RenderTexture2D) -> Self {
        let l = tf.len();
        let mut canvas = CanvasBuilder::init(tf)
            .size(l, 1)
            .cursor_position(0, 0)
            .build_no_attrs();

        for (i, (id, _)) in canvas.iter_mut().enumerate() {
            *id = i as CharID;
        }

        Self(GuiCanvas { canvas, target })
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
        let (cells, mut rd) = self.0.begin_canvas_mode(rl, rt);
        let (w, h) = (8, 8);
        for (x, y, color, _) in cells {
            rd.draw_rectangle(x as i32 * h, y as i32 * w, w, h, color);
        }
    }
}

pub struct UserCanvas(GuiCanvas<TextmodeFont, Attr>);

impl UserCanvas {
    pub fn new(canvas: Canvas<TextmodeFont, Attr>) -> Self {
        Self(GuiCanvas { canvas, target })
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
        rl.draw_texture(&self.0, 150, 80, Color::WHITE);
    }
}
