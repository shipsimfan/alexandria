use crate::math::{Rect, number::IntoF32};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Convert this [`Rect`]'s elements into [`f32`]s
    pub const fn into_f32(self) -> Rect<f32>
    where
        T: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32)
    }
}
