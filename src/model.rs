use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
    slice::Iter,
};

pub struct Array2D<T>(Vec<T>, u16);

/// coordinate to index
#[macro_export]
macro_rules! ctoi {
    ($width:expr, $x:expr, $y:expr) => {
        {$y*$width + $x} as usize
    };
}

pub use ctoi;

impl<T: Default> Array2D<T> {
    pub fn new(width: u16, height: u16) -> Self {
        let area = { width * height } as usize;
        let mut data = Vec::with_capacity(area);
        for _ in 0..area {
            data.push(T::default());
        }
        Self(data, width)
    }
}

impl<T> Array2D<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn slice(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    pub fn index_to_coord(&self, i: usize) -> (u16, u16) {
        let i = i as u16;
        (i % self.1, i / self.1)
    }
}

impl<T> Index<[u16; 2]> for Array2D<T> {
    type Output = T;

    fn index(&self, index: [u16; 2]) -> &Self::Output {
        let [x, y] = index;
        &self.0[ctoi!(self.1, x, y)]
    }
}

impl<T> IndexMut<[u16; 2]> for Array2D<T> {
    fn index_mut(&mut self, index: [u16; 2]) -> &mut Self::Output {
        let [x, y] = index;
        &mut self.0[ctoi!(self.1, x, y)]
    }
}

#[cfg(test)]
mod array_2d_test {
    use super::Array2D;

    #[derive(Debug, Eq, PartialEq, Default)]
    enum MockType {
        #[default]
        A,
        B,
    }

    #[test]
    fn manipulation() {
        let mut a = Array2D::<MockType>::new(2, 2);
        assert_eq!(a.len(), 4);
        assert_eq!(a[[0, 0]], MockType::A);
        a[[0, 0]] = MockType::B;
        assert_eq!(a[[0, 0]], MockType::B);
    }
}

pub type CharID = u16;
/// Consumes a character ID and
pub trait Charset {
    type Item;
    fn get_char(&self, id: CharID) -> Self::Item;
}

pub struct CanvasModel<C, A = ()> {
    /// The full grid of items.
    data: Array2D<(CharID, A)>,

    /// Represents all possible values that can be placed on the Canvas.
    /// Meant to decouple the backend from the frontend, for example a TUI frontend
    /// might have just ASCII or UTF-8, but this raylib frontend has numerical IDs that
    /// correspond to characters.
    charset: C,
}

impl<T, C, A> CanvasModel<C, A>
where
    C: Charset<Item = T>,
    A: Default,
{
    pub fn new(width: u16, height: u16, charset: C) -> Self {
        Self {
            data: Array2D::new(width, height),
            charset,
        }
    }

    pub fn put(&mut self, x: u16, y: u16, value: CharID, attributes: A) {
        self.data[[x, y]] = (value, attributes);
    }

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
    pub fn cells(&self) -> impl Iterator<Item = (u16, u16, T, &A)> {
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
}

#[cfg(test)]
mod canvas_model_test {
    use super::*;
    use std::collections::HashMap;

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
        let mut canvas = CanvasModel {
            data: Array2D::<Cell>::new(8, 8),
            charset: MockCharset { map: flowers_map() },
        };

        assert_eq!(
            canvas.iter().collect::<Vec<&Cell>>(),
            vec![(0, Soil::Brown); 8 * 8].iter().collect::<Vec<&Cell>>()
        );

        canvas.put(4, 4, 1, Soil::Green);
        assert_eq!(canvas.get(4, 4), &(1, Soil::Green));
    }
}
