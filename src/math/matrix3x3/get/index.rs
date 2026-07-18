use crate::math::{Matrix3x3, Vector3};
use std::ops::{Index, IndexMut};

const impl<T> Index<usize> for Matrix3x3<T> {
    type Output = Vector3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_row_slices()[index]
    }
}

const impl<T> IndexMut<usize> for Matrix3x3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_row_slices_mut()[index]
    }
}
