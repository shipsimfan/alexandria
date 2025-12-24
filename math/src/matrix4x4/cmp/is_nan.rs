use crate::{Matrix4x4, number::IsNaN};

impl<T> Matrix4x4<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.r0.is_nan() || self.r1.is_nan() || self.r2.is_nan() || self.r3.is_nan()
    }
}
