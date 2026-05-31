use crate::math::{ColorHsva, ColorSpace};
use std::ops::{Index, IndexMut};

impl<T, Space: ColorSpace<T>> Index<usize> for ColorHsva<T, Space> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.h,
            1 => &self.s,
            2 => &self.v,
            3 => &self.a,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl<T, Space: ColorSpace<T>> IndexMut<usize> for ColorHsva<T, Space> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.h,
            1 => &mut self.s,
            2 => &mut self.v,
            3 => &mut self.a,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}
