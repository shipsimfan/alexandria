use crate::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Convert this matrix into a iterator over the flattened columns
    pub fn into_flat_col_iter(self) -> std::array::IntoIter<T, 9> {
        self.into_flat_col_array().into_iter()
    }
}
