use crate::math::{ColorHsv, ColorSpace};
use std::ops::{Index, IndexMut};

impl<T, Space: ColorSpace<T>> Index<usize> for ColorHsv<T, Space> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.h,
            1 => &self.s,
            2 => &self.v,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl<T, Space: ColorSpace<T>> IndexMut<usize> for ColorHsv<T, Space> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.h,
            1 => &mut self.s,
            2 => &mut self.v,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}
