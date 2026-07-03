use crate::math::{Rect, Rectf, number::IntoF32};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Convert this [`Rect`]'s elements into [`f32`]s
    pub const fn into_f32(self) -> Rectf
    where
        P: [const] IntoF32 + [const] Destruct,
        S: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32, IntoF32::into_f32)
    }
}
