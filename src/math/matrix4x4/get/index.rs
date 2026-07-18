use crate::math::{Matrix4x4, Vector4};
use std::ops::{Index, IndexMut};

const impl<T> Index<usize> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_row_slices()[index]
    }
}

const impl<T> IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_row_slices_mut()[index]
    }
}
