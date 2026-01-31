use crate::math::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Convert this matrix into an iterator over the columns
    pub fn into_col_iter(self) -> std::array::IntoIter<Vector4<T>, 4> {
        self.into_vec4_col_array().into_iter()
    }
}
