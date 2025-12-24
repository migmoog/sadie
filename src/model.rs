mod actions;
mod array2d;
use array2d::Array2D;

use euclid::default::{Point2D, Size2D};

pub type CharID = u16;

/// Keeps track of Characters for drawing textmode art
pub trait Charset: Clone {
    type Item;

    /// Return the character corresponding to the id
    fn get_char(&self, id: CharID) -> Self::Item;

    /// Returns the number of characters in this set
    fn len(&self) -> u16;
}

pub type Position = Point2D<u16>;
pub struct Cursor {
    origin: Option<Position>,
    position: Position,
    bounds: Size2D<u16>,
}
impl Cursor {
    fn new(position: Position, bottom_bound: u16, right_bound: u16) -> Self {
        Self {
            origin: None,
            position,
            bounds: (bottom_bound, right_bound).into(),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

pub struct Canvas<C, A = ()> {
    /// The full grid of items.
    data: Array2D<(CharID, A)>,

    /// Represents all possible values that can be placed on the Canvas.
    /// Meant to decouple the backend from the frontend, for example a TUI frontend
    /// might have just ASCII or UTF-8, but this raylib frontend has numerical IDs that
    /// correspond to characters.
    charset: C,

    /// Canvas may have multiple cursors for doing actions. Examples:
    ///  - Font canvas has a cursor for picking a character
    ///  - Palette canvas has a cursor for picking a character
    cursors: Vec<Cursor>,
}

pub struct CanvasBuilder<C> {
    size: Option<Size2D<u16>>,
    charset: C,
    cursor_positions: Vec<Position>,
}

impl<T, C> CanvasBuilder<C>
where
    C: Charset<Item = T>,
{
    pub fn init(charset: C) -> Self {
        Self {
            size: None,
            cursor_positions: vec![],
            charset,
        }
    }

    /// Specifies the dimensions of the canvas
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.size = Some((width, height).into());
        self
    }

    /// Pushes a cursor onto the canvas
    pub fn cursor_position(mut self, x: u16, y: u16) -> Self {
        self.cursor_positions.push((x, y).into());
        self
    }

    pub fn build<A: Default>(self) -> Canvas<C, A> {
        let (width, height) = self.size.map(|s| s.into()).unwrap_or((8, 8));
        let mut cursors = self.cursor_positions;
        if cursors.is_empty() {
            cursors.push((0, 0).into());
        }
        let cursors = cursors
            .into_iter()
            .map(|p| Cursor::new(p, width, height))
            .collect();

        Canvas {
            data: Array2D::new(width, height),
            charset: self.charset,
            cursors,
        }
    }

    pub fn build_no_attrs(self) -> Canvas<C> {
        self.build::<()>()
    }
}

pub type Cell<'a, T, A> = (u16, u16, T, &'a A);

impl<T, C, A> Canvas<C, A>
where
    C: Charset<Item = T>,
{
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(CharID, A)> {
        self.data.slice().iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (CharID, A)> {
        self.data.mut_slice().iter_mut()
    }

    pub fn get(&self, x: u16, y: u16) -> &(CharID, A) {
        &self.data[[x, y]]
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut (CharID, A) {
        &mut self.data[[x, y]]
    }

    /// Returns an iter of cells
    pub fn cells(&self) -> impl Iterator<Item = Cell<T, A>> {
        self.data
            .slice()
            .iter()
            .enumerate()
            .map(|(index, (id, attributes))| {
                let (x, y) = self.data.index_to_coord(index);
                (x, y, self.charset.get_char(*id), attributes)
            })
    }

    pub fn charset(&self) -> &C {
        &self.charset
    }

    pub fn size(&self) -> Size2D<u16> {
        self.data.sides()
    }

    pub fn set_size(&mut self, new_size: Size2D<u16>) {
        self.data.set_width(new_size.width);

        for c in self.cursors.iter_mut() {
            c.bounds = new_size;
        }
    }
}

#[cfg(test)]
mod canvas_model_test {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct MockCharset<T> {
        map: HashMap<CharID, T>,
    }

    #[derive(Clone)]
    enum Flowers {
        Dandelion,
        Rose,
        Peony,
        Mums,
    }

    impl<T: Clone> Charset for MockCharset<T> {
        type Item = T;
        fn get_char(&self, id: CharID) -> Self::Item {
            self.map.get(&id).unwrap().clone()
        }

        fn len(&self) -> u16 {
            self.map.len() as u16
        }
    }

    fn flowers_map() -> HashMap<CharID, Flowers> {
        HashMap::from([
            (0, Flowers::Dandelion),
            (1, Flowers::Rose),
            (2, Flowers::Peony),
            (3, Flowers::Mums),
        ])
    }

    #[derive(Debug, Default, Clone, Eq, PartialEq)]
    enum Soil {
        #[default]
        Brown,
        Black,
        Green,
    }

    type Cell = (u16, Soil);

    #[test]
    fn putting_in_attributes() {
        let mut canvas = Canvas {
            data: Array2D::<Cell>::new(8, 8),
            charset: MockCharset { map: flowers_map() },
            cursors: vec![],
        };

        assert_eq!(
            canvas.iter().collect::<Vec<&Cell>>(),
            vec![(0, Soil::Brown); 8 * 8].iter().collect::<Vec<&Cell>>()
        );

        canvas.get_mut(4, 4).1 = Soil::Green;
        assert_eq!(canvas.get(4, 4), &(1, Soil::Green));
    }
}
