use crate::math::{Vector2, number::FromF32};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a [`Vector2`] from one made of [`f32`]s
    pub const fn from_f32(vector: Vector2<f32>) -> Vector2<T>
    where
        T: [const] FromF32 + [const] Destruct,
    {
        vector.map(FromF32::from_f32)
    }
}
