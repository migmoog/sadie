pub mod actions;
mod array2d;
pub mod canvas;
pub mod gallery;

use euclid::default::Point2D;

pub type CharID = u16;

/// Keeps track of Characters for drawing textmode art
pub trait Charset: Clone {
    /// Data to be returned when providing a `CharID`
    type Item;

    /// Return the character corresponding to the id
    fn get_char(&self, id: CharID) -> Self::Item;

    /// Returns the number of characters in this set
    fn len(&self) -> u16;
}

pub type CanvasPos = Point2D<u16>;
