use crate::math::{Rect, number::FromF32};
use std::marker::Destruct;

impl Rect<f32> {
    /// Create a [`Rect`] from one made of [`f32`]s
    pub const fn from_f32<T>(self) -> Rect<T>
    where
        T: [const] FromF32 + [const] Destruct,
    {
        self.map(FromF32::from_f32)
    }
}
