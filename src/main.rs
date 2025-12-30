use clap::Parser;
use raylib::prelude::*;
use thiserror::Error;

use crate::{core::actions::{Action, parse_action}, gui::RaylibContext};

mod core;
mod gui;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tui: bool,
}

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

/// Something controls the flow and drawing of sadie
pub trait SadieContext {
    fn check_input(&mut self) -> Option<char>;
    fn is_alive(&self) -> bool;
    fn draw(&mut self);
    fn apply_actions(&mut self, action: Action) -> Result<(), SadieError>;
}

fn run<T: SadieContext>(mut context: T) -> Result<(), SadieError> {
    let mut action_buffer = String::new();
    while context.is_alive() {
        // Update logic here
        if let Some(c) = context.check_input() {
            action_buffer.push(c);
            if let Some(Ok(a)) = parse_action(&action_buffer) {
                context.apply_actions(a)?;
                action_buffer.clear();
            }
        }

        // drawing logic here
        context.draw();
    }

    Ok(())
}

fn main() -> Result<(), SadieError> {
    let args = Args::parse();

    run(if args.tui { todo!() } else { RaylibContext::default() })
}
