use crate::{Matrix3x3, Vector3};
use std::ops::{Index, IndexMut};

impl<T> const Index<usize> for Matrix3x3<T> {
    type Output = Vector3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_row_slices()[index]
    }
}

impl<T> const IndexMut<usize> for Matrix3x3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_row_slices_mut()[index]
    }
}
