use crate::Vector2;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}
