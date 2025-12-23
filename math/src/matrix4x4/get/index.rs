use crate::{Matrix4x4, Vector4};
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        match self.row_ref(index) {
            Some(row) => row,
            None => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.row_mut_ref(index) {
            Some(row) => row,
            None => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}
