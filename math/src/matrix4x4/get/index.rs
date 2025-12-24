use crate::{Matrix4x4, Vector4};
use std::ops::{Index, IndexMut};

impl<T> const Index<usize> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_row_slices()[index]
    }
}

impl<T> const IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_row_slices_mut()[index]
    }
}
