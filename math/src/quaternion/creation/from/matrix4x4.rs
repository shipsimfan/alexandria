use crate::{
    Matrix4x4, Quaternion,
    number::{FromF32, One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + FromF32
        + Sqrt
        + Clone
        + PartialOrd
        + Zero
        + One,
> Quaternion<T>
{
    /// Create a new [`Quaternion`] from a rotation `matrix`, assuming it has orthonormal basis
    /// vectors
    pub fn from_matrix4x4(matrix: Matrix4x4<T>) -> Quaternion<T> {
        matrix.rotation()
    }
}

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + FromF32
        + Sqrt
        + Clone
        + PartialOrd
        + Zero
        + One,
> From<Matrix4x4<T>> for Quaternion<T>
{
    fn from(matrix: Matrix4x4<T>) -> Self {
        Quaternion::from_matrix4x4(matrix)
    }
}
