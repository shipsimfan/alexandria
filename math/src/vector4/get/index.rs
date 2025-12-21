use crate::Vector4;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Vector4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}
