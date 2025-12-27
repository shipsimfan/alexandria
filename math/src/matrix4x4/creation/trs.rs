use crate::{
    Matrix4x4, Quaternion, Vector3,
    number::{FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from `translation`, `rotation`, and `scale`
    pub const fn from_trs(
        translation: Vector3<T>,
        rotation: Quaternion<T>,
        scale: Vector3<T>,
    ) -> Matrix4x4<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        Matrix4x4::from_translation(translation)
            * Matrix4x4::from_rotation(rotation)
            * Matrix4x4::from_scale(scale)
    }
}
