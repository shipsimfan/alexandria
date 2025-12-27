use crate::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Convert this matrix into an iterator over the columns
    pub fn into_col_iter(self) -> std::array::IntoIter<Vector3<T>, 3> {
        self.into_vec3_col_array().into_iter()
    }
}
