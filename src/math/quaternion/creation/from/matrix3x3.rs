use crate::math::{
    Matrix3x3, Quaternion,
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
    pub fn from_matrix3x3(matrix: Matrix3x3<T>) -> Quaternion<T> {
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
> From<Matrix3x3<T>> for Quaternion<T>
{
    fn from(matrix: Matrix3x3<T>) -> Self {
        Quaternion::from_matrix3x3(matrix)
    }
}
