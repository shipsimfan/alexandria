use crate::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Convert the elements of this [`Matrix3x3`] into a row-major array
    pub const fn into_vec3_row_array(self) -> [Vector3<T>; 3]
    where
        T: [const] Destruct,
    {
        [self.r0, self.r1, self.r2]
    }

    /// Convert the elements of this [`Matrix3x3`] into a column-major array
    pub const fn into_vec3_col_array(self) -> [Vector3<T>; 3]
    where
        T: [const] Destruct,
    {
        [
            Vector3::new(self.r0.x, self.r1.x, self.r2.x),
            Vector3::new(self.r0.y, self.r1.y, self.r2.y),
            Vector3::new(self.r0.z, self.r1.z, self.r2.z),
        ]
    }
}

impl<T: [const] Destruct> const Into<[Vector3<T>; 3]> for Matrix3x3<T> {
    fn into(self) -> [Vector3<T>; 3] {
        self.into_vec3_row_array()
    }
}
