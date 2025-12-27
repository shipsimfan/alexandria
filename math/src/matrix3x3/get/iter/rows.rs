use crate::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Convert this matrix into an iterator over the rows
    pub fn into_row_iter(self) -> std::array::IntoIter<Vector3<T>, 3> {
        self.into_vec3_row_array().into_iter()
    }

    /// Get an iterator over the rows
    pub const fn row_iter<'a>(&'a self) -> std::slice::Iter<'a, Vector3<T>> {
        self.as_row_slices().iter()
    }

    /// Get a mutable iterator over the rows
    pub const fn row_iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Vector3<T>> {
        self.as_row_slices_mut().iter_mut()
    }
}

impl<T> IntoIterator for Matrix3x3<T> {
    type Item = Vector3<T>;
    type IntoIter = std::array::IntoIter<Vector3<T>, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_row_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix3x3<T> {
    type Item = &'a Vector3<T>;
    type IntoIter = std::slice::Iter<'a, Vector3<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix3x3<T> {
    type Item = &'a mut Vector3<T>;
    type IntoIter = std::slice::IterMut<'a, Vector3<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row_iter_mut()
    }
}
