use crate::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] from [`Vector3`] rows
    pub const fn from_rows([r0, r1, r2]: [Vector3<T>; 3]) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(r0, r1, r2)
    }

    /// Create a new [`Matrix3x3`] from [`Vector3`] columns
    pub const fn from_cols([c0, c1, c2]: [Vector3<T>; 3]) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_cols(c0, c1, c2)
    }
}

impl<T: [const] Destruct> const From<[Vector3<T>; 3]> for Matrix3x3<T> {
    fn from(rows: [Vector3<T>; 3]) -> Self {
        Matrix3x3::from_rows(rows)
    }
}
