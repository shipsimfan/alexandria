use crate::math::{
    Matrix3x3, Quaternion, Vector3,
    number::{FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] from `rotation` and `scale`
    pub const fn from_rotation_scale(rotation: Quaternion<T>, scale: Vector3<T>) -> Matrix3x3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        Matrix3x3::from_rotation(rotation) * Matrix3x3::from_scale(scale)
    }
}
