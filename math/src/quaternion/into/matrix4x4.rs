use crate::{
    Matrix4x4, Quaternion,
    number::{FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Quaternion<T> {
    /// Convert this [`Quaternion`] into a [`Matrix4x4`]
    pub const fn into_mat4(self) -> Matrix4x4<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct
            + Zero
            + One,
    {
        Matrix4x4::from_rotation(self)
    }
}
