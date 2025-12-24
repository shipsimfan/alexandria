use crate::Vector4;
use std::ops::{Index, IndexMut};

impl<T> const Index<usize> for Vector4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> const IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}
