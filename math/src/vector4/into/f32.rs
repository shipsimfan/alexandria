use crate::{Vector4, number::FromF32};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Create a [`Vector4`] from one made of [`f32`]s
    pub const fn from_f32(vector: Vector4<f32>) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        vector.map(FromF32::from_f32)
    }
}
