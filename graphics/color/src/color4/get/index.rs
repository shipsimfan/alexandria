use crate::{Color4, ColorSpace};
use std::ops::{Index, IndexMut};

impl<T, Space: ColorSpace<T>> Index<usize> for Color4<T, Space> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl<T, Space: ColorSpace<T>> IndexMut<usize> for Color4<T, Space> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}
