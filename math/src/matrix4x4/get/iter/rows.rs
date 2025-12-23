use crate::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Convert this matrix into an iterator over the rows
    pub fn into_row_iter(self) -> std::array::IntoIter<Vector4<T>, 4> {
        self.into_vec4_row_array().into_iter()
    }

    /// Get an iterator over the rows
    pub const fn row_iter<'a>(&'a self) -> std::slice::Iter<'a, Vector4<T>> {
        self.as_row_slices().iter()
    }

    /// Get a mutable iterator over the rows
    pub const fn row_iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Vector4<T>> {
        self.as_row_slices_mut().iter_mut()
    }
}

impl<T> IntoIterator for Matrix4x4<T> {
    type Item = Vector4<T>;
    type IntoIter = std::array::IntoIter<Vector4<T>, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_row_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix4x4<T> {
    type Item = &'a Vector4<T>;
    type IntoIter = std::slice::Iter<'a, Vector4<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix4x4<T> {
    type Item = &'a mut Vector4<T>;
    type IntoIter = std::slice::IterMut<'a, Vector4<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row_iter_mut()
    }
}
