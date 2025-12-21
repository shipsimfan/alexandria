use crate::{Vector4, number::IntoF32};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Convert this vector's elements into [`f32`]s
    pub const fn into_f32(self) -> Vector4<f32>
    where
        T: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32)
    }
}
