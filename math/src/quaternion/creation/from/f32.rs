use crate::{Quaternion, number::FromF32};
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Create a [`Quaternion`] from one made of [`f32`]s
    pub const fn from_f32(vector: Quaternion<f32>) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        vector.map(FromF32::from_f32)
    }
}
