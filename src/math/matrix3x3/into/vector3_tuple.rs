use crate::math::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Convert the elements of this [`Matrix3x3`] into a row-major tuple
    pub const fn into_vec3_row_tuple(self) -> (Vector3<T>, Vector3<T>, Vector3<T>)
    where
        T: [const] Destruct,
    {
        (self.r0, self.r1, self.r2)
    }

    /// Convert the elements of this [`Matrix3x3`] into a column-major tuple
    pub const fn into_vec3_col_tuple(self) -> (Vector3<T>, Vector3<T>, Vector3<T>)
    where
        T: [const] Destruct,
    {
        (
            Vector3::new(self.r0.x, self.r1.x, self.r2.x),
            Vector3::new(self.r0.y, self.r1.y, self.r2.y),
            Vector3::new(self.r0.z, self.r1.z, self.r2.z),
        )
    }
}

impl<T: [const] Destruct> const Into<(Vector3<T>, Vector3<T>, Vector3<T>)> for Matrix3x3<T> {
    fn into(self) -> (Vector3<T>, Vector3<T>, Vector3<T>) {
        self.into_vec3_row_tuple()
    }
}
