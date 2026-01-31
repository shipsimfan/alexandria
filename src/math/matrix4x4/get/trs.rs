use crate::math::{
    Matrix4x4, Quaternion, Vector3,
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
> Matrix4x4<T>
{
    /// Extract translation, rotation, and scale from this matrix
    pub fn trs(self) -> (Vector3<T>, Quaternion<T>, Vector3<T>) {
        (
            self.clone().translation(),
            self.clone().rotation(),
            self.scale(),
        )
    }
}
