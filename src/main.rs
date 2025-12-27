use raylib::prelude::*;
use thiserror::Error;

use crate::gui::Client;

mod gui;
mod core;

#[derive(Error, Debug)]
pub enum SadieError {
    #[error("Cannot find file \"{:?}\"", path)]
    CantFindFile { path: String },

    #[error(
        "Image fonts must be only black and white.
Palette: {:?}
in {:?}",
        palette,
        fontname
    )]
    NotBlackAndWhite {
        fontname: String,
        palette: ImagePalette,
    },

    #[error("Raylib: {0:?}")]
    Raylib(raylib::core::error::Error),
}

fn main() -> Result<(), SadieError> {
    Client::new()?.run()?;
    Ok(())
}
