use crate::math::{Rect, Rectf, number::FromF32};
use std::marker::Destruct;

impl Rectf {
    /// Create a [`Rect`] from one made of [`f32`]s
    pub const fn from_f32<P, S>(self) -> Rect<P, S>
    where
        P: [const] FromF32 + [const] Destruct,
        S: [const] FromF32 + [const] Destruct,
    {
        self.map(FromF32::from_f32, FromF32::from_f32)
    }
}
