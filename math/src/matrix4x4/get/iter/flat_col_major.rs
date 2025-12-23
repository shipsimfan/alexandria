use crate::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Convert this matrix into a iterator over the flattened columns
    pub fn into_flat_col_iter(self) -> std::array::IntoIter<T, 16> {
        self.into_flat_col_array().into_iter()
    }
}
