use crate::Vector4;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Vector4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get_ref(index) {
            Some(value) => value,
            None => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.get_ref_mut(index) {
            Some(value) => value,
            None => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}
