use crate::math::{Matrix3x3, number::IntoF32};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Convert this matrix's elements into [`f32`]s
    pub const fn into_f32(self) -> Matrix3x3<f32>
    where
        T: [const] IntoF32 + [const] Destruct,
    {
        self.map(IntoF32::into_f32)
    }
}
