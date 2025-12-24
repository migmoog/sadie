use crate::{
    gui::{font::TextmodeFont, GuiCharset},
    model::{Canvas, CanvasBuilder, Cell, CharID, Charset, Position},
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

    pub fn draw<Rd>(&mut self, p: Position, rd: &mut Rd, rt: &RaylibThread)
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
    p: Position,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    {
        let src = gc.model.charset().source.clone();
        let (cells, mut rd) = gc.begin_canvas_mode(rl, rt);

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

    rl.draw_texture(&gc, p.x.into(), p.y.into(), Color::WHITE);
}

fn draw_color_picker<Rd>(
    canvas: &mut GuiCanvas<Palette>,
    p: Position,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    {
        let (cells, mut rd) = canvas.begin_canvas_mode(rl, rt);
        let (w, h) = (8, 8);
        for (x, y, color, _) in cells {
            rd.draw_rectangle(x as i32 * h, y as i32 * w, w, h, color);
        }
    }

    rl.draw_texture(&canvas, p.x.into(), p.y.into(), Color::WHITE);
}

fn draw_user_canvas<Rd>(
    canvas: &mut GuiCanvas<TextmodeFont, Attr>,
    p: Position,
    rl: &mut Rd,
    rt: &RaylibThread,
) where
    Rd: RaylibDraw + RaylibTextureModeExt,
{
    {
        let src = canvas.model.charset().source.clone();
        let (cells, mut rd) = canvas.begin_canvas_mode(rl, rt);
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

impl<T, C, A> GuiCanvas<C, A>
where
    C: GuiCharset<Item = T>,
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
    fn begin_canvas_mode<'a, Rd>(
        &'a mut self,
        rl: &'a mut Rd,
        rt: &'a RaylibThread,
    ) -> (impl Iterator<Item = Cell<T, A>>, RaylibTextureMode<'a, Rd>)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        (
            self.model.cells(),
            rl.begin_texture_mode(rt, &mut self.target),
        )
    }
}

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
