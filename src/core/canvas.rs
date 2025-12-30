use super::{array2d::Array2D, CharID, Charset};

use euclid::default::Size2D;

use super::CanvasPos;

pub struct Cursor {
    origin: Option<CanvasPos>,
    position: CanvasPos,
    bounds: Size2D<u16>,
}

impl Cursor {
    pub(crate) fn new(position: CanvasPos, bottom_bound: u16, right_bound: u16) -> Self {
        Self {
            origin: None,
            position,
            bounds: (bottom_bound, right_bound).into(),
        }
    }

    pub fn position(&self) -> CanvasPos {
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

pub struct CanvasBuilder<C, A = ()> {
    size: Size2D<u16>,
    charset: C,
    cursor_positions: Vec<CanvasPos>,
    default_cells: Option<Vec<(CharID, A)>>,
}

impl<T, C, A> CanvasBuilder<C, A>
where
    C: Charset<Item = T>,
    A: Default,
{
    pub fn init(charset: C) -> Self {
        Self {
            size: (charset.len(), 1).into(),
            cursor_positions: vec![],
            charset,
            default_cells: None,
        }
    }

    /// Specifies the dimensions of the canvas
    pub fn size(mut self, size: Size2D<u16>) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.size.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.size.height = height;
        self
    }

    /// Pushes a cursor onto the canvas
    pub fn cursor_position(mut self, x: u16, y: u16) -> Self {
        self.cursor_positions.push((x, y).into());
        self
    }

    /// Uses a function to create the default members of a grid
    pub fn default_cells<F>(mut self, func: F) -> Self
    where
        F: Fn(CharID, &C) -> (CharID, A),
    {
        self.default_cells = Some(
            (0..self.size.area())
                .map(|id| func(id, &self.charset))
                .collect(),
        );
        self
    }

    /// Sets the first characters of a charset to be each character of the charset
    pub fn char_cascade(self) -> Self {
        self.default_cells(|id, c| (if id < c.len() { id } else { 0 }, A::default()))
    }

    pub fn build(self) -> Canvas<C, A> {
        let (width, height) = self.size.into();
        let mut cursors = self.cursor_positions;
        if cursors.is_empty() {
            cursors.push((0, 0).into());
        }
        let cursors = cursors
            .into_iter()
            .map(|p| Cursor::new(p, width, height))
            .collect();

        let data = if let Some(default_cells) = self.default_cells {
            (default_cells, width).into()
        } else {
            Array2D::new(width, height)
        };

        Canvas {
            data,
            charset: self.charset,
            cursors,
        }
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

    /// Sets the new size, and updates the bounds of cursors
    pub fn set_size(&mut self, new_size: Size2D<u16>) {
        self.data.set_width(new_size.width);

        for c in self.cursors.iter_mut() {
            c.bounds = new_size;
        }
    }

    /// Returns state of cursors on the grid. Contents are:
    /// - An optional icon to give information on how to draw the cursor
    /// - and iterator of all the cursor's positions
    pub fn cursors(&self) -> impl Iterator<Item = &Cursor> {
        self.cursors.iter()
    }
}

#[cfg(test)]
pub(crate) mod canvas_model_test {
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
