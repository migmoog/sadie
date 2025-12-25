use crate::{
    gui::{font::TextmodeFont, GuiCharset},
    model::{Canvas, CanvasBuilder, CanvasPos, CharID, Charset, Cursor},
};
use euclid::default::Size2D;
use raylib::prelude::*;

pub enum GuiComponent {
    UserCanvas(GuiCanvas<TextmodeFont, Attr>),
    CharsetPicker(GuiCanvas<TextmodeFont>),
    ColorPicker(GuiCanvas<Palette>),
}

impl GuiComponent {
    pub fn make_charset_picker(tf: TextmodeFont, rl: &mut RaylibHandle, rt: &RaylibThread) -> Self {
        let l = tf.len();
        let mut canvas = CanvasBuilder::init(tf)
            .size(l, 1)
            .cursor_position(0, 0)
            .build_no_attrs();

        for (i, (id, _)) in canvas.iter_mut().enumerate() {
            *id = i as CharID;
        }

        Self::CharsetPicker(GuiCanvas::with_target(canvas, rl, rt))
    }

    pub fn make_color_picker(p: Palette, rl: &mut RaylibHandle, rt: &RaylibThread) -> Self {
        let l = p.len();
        let mut canvas = CanvasBuilder::init(p)
            .size(l, 1)
            .cursor_position(0, 0)
            .cursor_position(1, l)
            .build_no_attrs();

        for (i, (id, _)) in canvas.iter_mut().enumerate() {
            *id = i as CharID;
        }

        Self::ColorPicker(GuiCanvas::with_target(canvas, rl, rt))
    }

    pub fn make_user_canvas(
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        canvas: Canvas<TextmodeFont, Attr>,
    ) -> Self {
        let mut gc = GuiCanvas::with_target(canvas, rl, rt);
        let l = gc.model.len();
        for (id, attr) in gc.model.iter_mut() {
            *id = rl.get_random_value::<i32>(0..{ l as i32 }) as CharID;
            let clrs = [Color::RED, Color::BLUE, Color::GREEN];

            let clrl = clrs.len() as i32;
            let fgi = rl.get_random_value::<i32>(0..clrl - 1) as usize;
            attr.fg = clrs[fgi];
            let bgi = rl.get_random_value::<i32>(0..clrl - 1) as usize;
            attr.bg = clrs[bgi];
        }

        Self::UserCanvas(gc)
    }

    pub fn draw<Rd>(&mut self, p: CanvasPos, rd: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        match self {
            Self::UserCanvas(c) => draw_user_canvas(c, p, rd, rt),
            Self::ColorPicker(c) => draw_color_picker(c, p, rd, rt),
            Self::CharsetPicker(c) => draw_charset_picker(c, p, rd, rt),
        }
    }
}

fn draw_charset_picker<Rd>(
    gc: &mut GuiCanvas<TextmodeFont>,
    p: CanvasPos,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    let src = gc.model.charset().source.clone();
    gc.draw_cells_mode(rl, rt, |d, p, r, _| {
        d.draw_texture_rec(
            &src,
            r,
            Vector2 {
                x: p.x as f32 * r.width,
                y: p.y as f32 * r.height,
            },
            Color::WHITE,
        );
    });

    rl.draw_texture(&gc, p.x.into(), p.y.into(), Color::WHITE);
}

fn draw_color_picker<Rd>(
    canvas: &mut GuiCanvas<Palette>,
    p: CanvasPos,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    let s = canvas.model.charset().get_char_size();
    canvas.draw_cells_mode(rl, rt, |d, p, t, _| {
        let (w, h) = (s.width as i32, s.height as i32);
        d.draw_rectangle(p.x as i32 * h, p.y as i32 * w, w, h, t);
    });

    rl.draw_texture(&canvas, p.x.into(), p.y.into(), Color::WHITE);
}

fn draw_user_canvas<Rd>(
    canvas: &mut GuiCanvas<TextmodeFont, Attr>,
    p: CanvasPos,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    let src = canvas.model.charset().source.clone();
    canvas.draw_cells_mode(rl, rt, |d, p, r, _| {
        d.draw_texture_rec(
            &src,
            r,
            Vector2 {
                x: p.x as f32 * r.width,
                y: p.y as f32 * r.height,
            },
            Color::WHITE,
        );
    });

    rl.draw_texture(&canvas, p.x.into(), p.y.into(), Color::WHITE);
}

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

pub struct GuiCanvas<C, A = ()> {
    model: Canvas<C, A>,
    target: RenderTexture2D,
}

impl<C, A> AsRef<raylib::ffi::Texture> for GuiCanvas<C, A> {
    fn as_ref(&self) -> &raylib::ffi::Texture {
        self.target.as_ref()
    }
}

impl<C, A> GuiCanvas<C, A>
where
    C: GuiCharset,
{
    fn with_target(canvas: Canvas<C, A>, rl: &mut RaylibHandle, rt: &RaylibThread) -> Self {
        let size = canvas.charset().get_char_size();
        let s = canvas.size();

        let target = rl
            .load_render_texture(
                &rt,
                size.width as u32 * s.width as u32,
                size.height as u32 * s.height as u32,
            )
            .expect("couldn't make render texture");

        Self {
            model: canvas,
            target,
        }
    }
}

// welcome to GENERICS HELL 😈
impl<T, C, A> GuiCanvas<C, A>
where
    C: Charset<Item = T>,
{
    /// Takes a closure to draw all cells on the canvas
    fn draw_cells_mode<Rd, F>(&mut self, rl: &mut Rd, rt: &RaylibThread, mut callback: F)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
        F: FnMut(&mut Rd, CanvasPos, T, &A),
    {
        let mut d = rl.begin_texture_mode(rt, &mut self.target);

        for (x, y, t, a) in self.model.cells() {
            callback(&mut d, (x, y).into(), t, a);
        }
    }

    /// Takes a closure to draw all cursors atop the canvas
    fn draw_cursors_mode<Rd, F>(&mut self, rl: &mut Rd, rt: &RaylibThread, mut callback: F)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
        F: FnMut(&mut Rd, &Cursor),
    {
        let mut d = rl.begin_texture_mode(rt, &mut self.target);

        for c in self.model.cursors() {
            callback(&mut d, c);
        }
    }
}

