use std::ops::{Index, IndexMut};

use euclid::default::Size2D;

pub struct Array2D<T>(Vec<T>, u16);

/// coordinate to index
#[macro_export]
macro_rules! ctoi {
    ($width:expr, $x:expr, $y:expr) => {
        {$y*$width + $x} as usize
    };
}

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

    pub fn sides(&self) -> Size2D<u16> {
        let height = self.0.len() as u16 / self.1;
        (self.1, height).into()
    }

    pub fn set_width(&mut self, width: u16) {
        self.1 = width;
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

impl<T> From<( Vec<T>,  u16)> for Array2D<T> {
    fn from(value: ( Vec<T>, u16 )) -> Self {
        let (cells, width) = value;
        Self(cells, width)
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
