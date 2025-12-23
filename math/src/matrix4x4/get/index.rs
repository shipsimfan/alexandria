use crate::{Matrix4x4, Vector4};
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Matrix4x4<T> {
    type Output = Vector4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.row_ref(index)
    }
}

impl<T> IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.row_mut_ref(index)
    }
}
