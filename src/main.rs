use std::path::Path;
use raylib::texture::Texture2D;
use raylib::prelude::*;

#[derive(Debug)]
enum TextmigError {
    FontSrcDoesNotExist(String),
}
type TMResult<T> = Result<T, TextmigError>;
struct Grid {
    dimensions: (u16, u16),
    data: Vec<u8>, // flattened 2d array
    tmfont_src: Texture2D,
}

impl Grid {
    fn new(width: u16, height: u16, tmfont_src: Texture2D) -> Self {
       Self {
           dimensions: (width, height),
           data: Vec::with_capacity({width * height} as usize),
           tmfont_src,
       }
    }

    fn load(rl: &mut RaylibHandle, rt: &RaylibThread, path: &Path) -> TMResult<Self> {
       let tex = rl.load_texture(rt, path.to_str().expect("Path is not valid UTF-8"))
           .map_err(TextmigError::FontSrcDoesNotExist)?;

        Ok(Self::new(8, 8, tex))
    }
}

struct Env {
    rl: RaylibHandle,
    thread: RaylibThread,
    grid: Grid,
}

impl Env {
    fn new(path_to_font_src: &Path) -> TMResult<Self> {
        let (mut rl, thread) = init()
            .size(640, 480)
            .title("Textmig")
            .build();

        let grid = Grid::load(&mut rl, &thread, path_to_font_src)?;

        Ok(Self {
            rl,
            thread,
            grid
        })
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