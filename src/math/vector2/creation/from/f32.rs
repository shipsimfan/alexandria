use crate::math::{Vector2, number::FromF32};
use std::marker::Destruct;

impl Vector2<f32> {
    /// Create a [`Vector2`] from one made of [`f32`]s
    pub const fn from_f32<T>(self) -> Vector2<T>
    where
        T: [const] FromF32 + [const] Destruct,
    {
        self.map(FromF32::from_f32)
    }
}
