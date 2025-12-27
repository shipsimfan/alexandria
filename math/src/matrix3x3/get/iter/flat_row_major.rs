use crate::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Convert this matrix into an iterator over the flattened rows
    pub fn into_flat_row_iter(self) -> std::array::IntoIter<T, 9> {
        self.into_flat_row_array().into_iter()
    }

    /// Get an iterator over the flattened rows
    pub const fn flat_row_iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    /// Get a mutable iterator over the flattened rows
    pub const fn flat_row_iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
        self.as_slice_mut().iter_mut()
    }
}
