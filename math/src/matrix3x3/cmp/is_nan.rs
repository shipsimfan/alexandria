use crate::{Matrix3x3, number::IsNaN};

impl<T> Matrix3x3<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.r0.is_nan() || self.r1.is_nan() || self.r2.is_nan()
    }
}
