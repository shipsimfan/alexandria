use crate::math::{Vector4, number::FromF32};
use std::marker::Destruct;

impl Vector4<f32> {
    /// Create a [`Vector4`] from one made of [`f32`]s
    pub const fn from_f32<T>(self) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        self.map(FromF32::from_f32)
    }
}
