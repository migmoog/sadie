use raylib::prelude::*;
use raylib::texture::Texture2D;
use std::path::Path;

use crate::textgrid::Grid;

mod tmfont;
mod textgrid;

#[derive(Debug)]
pub enum SadieError {
    FontSrcDoesNotExist,
}
pub type TMResult<T> = Result<T, SadieError>;

struct Env {
    rl: RaylibHandle,
    thread: RaylibThread,
    grid: Grid,
}

impl Env {
    fn new(path_to_font_src: &Path) -> TMResult<Self> {
        let (mut rl, thread) = init().size(640, 480).title("Textmig").build();

        let grid = Grid::load(&mut rl, &thread, path_to_font_src)?;

        Ok(Self { rl, thread, grid })
    }

    fn run(&mut self) -> TMResult<()> {
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::BLACK);
        }

        Ok(())
    }
}

impl Default for Env {
    fn default() -> Self {
        let path_to_font_src = Path::new("gloop_8x8.png");
        Self::new(path_to_font_src).expect("MUST have gloop_8x8.png in order to run default")
    }
}

fn main() -> TMResult<()> {
    Env::default().run()
}
