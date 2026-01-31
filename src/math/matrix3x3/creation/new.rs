use crate::math::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] from [`Vector3`] rows
    pub const fn new_rows(r0: Vector3<T>, r1: Vector3<T>, r2: Vector3<T>) -> Matrix3x3<T> {
        Matrix3x3 { r0, r1, r2 }
    }

    /// Create a new [`Matrix3x3`] from [`Vector3`] columns
    pub const fn new_cols(c0: Vector3<T>, c1: Vector3<T>, c2: Vector3<T>) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(c0, c1, c2).transpose()
    }
}
