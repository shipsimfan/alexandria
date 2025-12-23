use crate::Vector3;
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get_ref(index) {
            Some(value) => value,
            None => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.get_ref_mut(index) {
            Some(value) => value,
            None => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}
