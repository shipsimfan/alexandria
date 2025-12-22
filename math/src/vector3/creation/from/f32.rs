use crate::{Vector3, number::IntoF32};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Convert this vector's elements into [`f32`]s
    pub const fn into_f32(self) -> Vector3<f32>
    where
        T: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32)
    }
}
