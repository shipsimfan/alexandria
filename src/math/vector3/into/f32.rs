use crate::math::{Vector3, number::FromF32};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Create a [`Vector3`] from one made of [`f32`]s
    pub const fn from_f32(vector: Vector3<f32>) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        vector.map(FromF32::from_f32)
    }
}
