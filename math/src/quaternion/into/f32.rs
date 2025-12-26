use crate::{Quaternion, number::IntoF32};
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Convert this vector's elements into [`f32`]s
    pub const fn into_f32(self) -> Quaternion<f32>
    where
        T: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32)
    }
}
