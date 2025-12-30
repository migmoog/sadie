use std::collections::{hash_map::Entry, HashMap};

use euclid::default::{Point2D, Size2D};
use raylib::prelude::*;

use crate::{
    core::{
        canvas::{Canvas, Cursor},
        gallery::Gallery,
    },
    gui::{font::TextmodeFont, palette::Palette, GuiCharset},
    SadieError,
};

// make my life a little easier :-)
impl<T, C, A> Canvas<C, A>
where
    C: GuiCharset<Item = T>,
{
    fn make_render_texture(
        &self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) -> Result<RenderTexture2D, SadieError> {
        let size = self.size();
        let size: Size2D<u32> = (size.width as u32, size.height as u32).into();
        self.charset().make_canvas_rtex(rl, rt, size)
    }

    fn draw_cells_mode<Rd: RaylibDraw, F>(&self, d: &mut Rd, mut func: F)
    where
        F: FnMut(&mut Rd, Point2D<u16>, T, &A),
    {
        for (x, y, t, a) in self.cells() {
            func(d, (x, y).into(), t, a);
        }
    }

    fn draw_cursors_mode<Rd, F>(&self, d: &mut Rd, mut func: F)
    where
        Rd: RaylibDraw,
        F: FnMut(&mut Rd, &Cursor),
    {
        for c in self.cursors() {
            func(d, c)
        }
    }
}

pub struct CellColors {
    pub fg: Color,
    pub bg: Color,
}

impl Default for CellColors {
    fn default() -> Self {
        Self {
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

/// A variant of a canvas and its charset that helps it
pub enum DrawableCanvas {
    ColoredFont(Canvas<TextmodeFont, CellColors>),
    FontOnly(Canvas<TextmodeFont>),
    ColorSquares(Canvas<Palette>),
}

/// Holds a canvas to draw, and it's position to be rendered at
pub struct Frame {
    pub position: Point2D<i32>,
    contents: DrawableCanvas,
    render_texture: RenderTexture2D,
}

impl Frame {
    fn draw<Rd>(&mut self, d: &mut Rd, rt: &RaylibThread)
    where
        Rd: RaylibDraw + RaylibTextureModeExt,
    {
        {
            let mut rd = d.begin_texture_mode(rt, &mut self.render_texture);
            match &self.contents {
                DrawableCanvas::ColoredFont(c) => {
                    c.draw_cells_mode(&mut rd, |rdd, p, r, a| {
                        rdd.draw_texture_rec(
                            c.charset(),
                            r,
                            Vector2 {
                                x: p.x as f32 * r.width,
                                y: p.y as f32 * r.height,
                            },
                            Color::WHITE,
                        );
                    });

                    let s = c.charset().get_char_size();
                    c.draw_cursors_mode(&mut rd, |rdd, c| draw_x_cursor(rdd, c, s));
                }
                DrawableCanvas::FontOnly(c) => {
                    c.draw_cells_mode(&mut rd, |rdd, p, r, _| {
                        rdd.draw_texture_rec(
                            c.charset(),
                            r,
                            Vector2 {
                                x: p.x as f32 * r.width,
                                y: p.y as f32 * r.height,
                            },
                            Color::WHITE,
                        );
                    });

                    let s = c.charset().get_char_size();
                    c.draw_cursors_mode(&mut rd, |rdd, c| draw_x_cursor(rdd, c, s));
                }
                DrawableCanvas::ColorSquares(c) => {
                    let size = c.charset().get_char_size();
                    c.draw_cells_mode(&mut rd, |rdd, p, t, _| {
                        let (w, h) = (size.width as i32, size.height as i32);
                        rdd.draw_rectangle(p.x as i32 * w, p.y as i32 * h, w, h, t);
                    });

                    let s = c.charset().get_char_size();
                    c.draw_cursors_mode(&mut rd, |rdd, c| draw_x_cursor(rdd, c, s));
                }
            }
        }

        d.draw_texture(
            &self.render_texture,
            self.position.x,
            self.position.y,
            Color::WHITE,
        );
    }
}

fn draw_x_cursor(d: &mut impl RaylibDraw, c: &Cursor, size: Size2D<u16>) {
    let p = c.position();
    let start: Point2D<i32> = ({ p.x * size.width } as i32, { p.y * size.height } as i32).into();
    let end: Point2D<i32> = (start.x + size.width as i32, start.y + size.height as i32).into();

    d.draw_line(start.x, start.y, end.x, end.y, Color::RED);
    d.draw_line(start.x, end.y, end.x, start.y, Color::RED);
}

impl
    TryFrom<(
        (&mut RaylibHandle, &RaylibThread),
        Canvas<TextmodeFont, CellColors>,
    )> for Frame
{
    type Error = SadieError;

    fn try_from(
        value: (
            (&mut RaylibHandle, &RaylibThread),
            Canvas<TextmodeFont, CellColors>,
        ),
    ) -> Result<Self, Self::Error> {
        let ((rl, rt), canvas) = value;

        let render_texture = canvas.make_render_texture(rl, rt)?;

        Ok(Self {
            position: Point2D::zero(),
            contents: DrawableCanvas::ColoredFont(canvas),
            render_texture,
        })
    }
}

impl TryFrom<((&mut RaylibHandle, &RaylibThread), Canvas<TextmodeFont>)> for Frame {
    type Error = SadieError;

    fn try_from(
        value: ((&mut RaylibHandle, &RaylibThread), Canvas<TextmodeFont>),
    ) -> Result<Self, Self::Error> {
        let ((rl, rt), canvas) = value;

        let render_texture = canvas.make_render_texture(rl, rt)?;

        Ok(Self {
            position: Point2D::zero(),
            contents: DrawableCanvas::FontOnly(canvas),
            render_texture,
        })
    }
}

impl TryFrom<((&mut RaylibHandle, &RaylibThread), Canvas<Palette>)> for Frame {
    type Error = SadieError;

    fn try_from(
        value: ((&mut RaylibHandle, &RaylibThread), Canvas<Palette>),
    ) -> Result<Self, Self::Error> {
        let ((rl, rt), canvas) = value;

        let render_texture = canvas.make_render_texture(rl, rt)?;

        Ok(Self {
            position: Point2D::zero(),
            contents: DrawableCanvas::ColorSquares(canvas),
            render_texture,
        })
    }
}

type CID = u32;
pub struct GuiGallery {
    id_base: CID,
    frames: HashMap<CID, Frame>,
}

impl GuiGallery {
    pub fn new() -> Self {
        Self {
            id_base: 1,
            frames: HashMap::new(),
        }
    }

    fn pick_id(&mut self) -> CID {
        let id = self.id_base;
        self.id_base += 1;
        id
    }

    pub fn add_colored_font(
        &mut self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        canvas: Canvas<TextmodeFont, CellColors>,
    ) -> Result<Entry<'_, CID, Frame>, SadieError> {
        let id = self.pick_id();
        let frame = ((rl, rt), canvas).try_into()?;
        self.frames.insert(id, frame);
        Ok(self.frames.entry(id))
    }

    pub fn add_font_only(
        &mut self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        canvas: Canvas<TextmodeFont>,
    ) -> Result<Entry<'_, CID, Frame>, SadieError> {
        let id = self.pick_id();
        let frame = ((rl, rt), canvas).try_into()?;
        self.frames.insert(id, frame);
        Ok(self.frames.entry(id))
    }

    pub fn add_color_squares(
        &mut self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
        canvas: Canvas<Palette>,
    ) -> Result<Entry<'_, CID, Frame>, SadieError> {
        let id = self.pick_id();
        let frame = ((rl, rt), canvas).try_into()?;
        self.frames.insert(id, frame);
        Ok(self.frames.entry(id))
    }

    pub fn draw<Rd: RaylibDraw + RaylibTextureModeExt>(&mut self, d: &mut Rd, rt: &RaylibThread) {
        for (_cid, frame) in self.frames.iter_mut() {
            frame.draw(d, rt);
        }
    }
}

impl Gallery for GuiGallery {
    type CanvasVariant = DrawableCanvas;
    type CanvasID = CID;

    fn all_ids(&self) -> impl Iterator<Item = Self::CanvasID> {
        self.frames.keys().map(|&id| id)
    }

    fn get_canvas(&self, id: Self::CanvasID) -> Option<&Self::CanvasVariant> {
        self.frames.get(&id).map(|f| &f.contents)
    }
}
