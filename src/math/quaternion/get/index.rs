use crate::math::Quaternion;
use std::ops::{Index, IndexMut};

const impl<T> Index<usize> for Quaternion<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

const impl<T> IndexMut<usize> for Quaternion<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}
