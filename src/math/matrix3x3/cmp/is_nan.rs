use crate::math::{Matrix3x3, number::IsNaN};

impl<T> Matrix3x3<T> {
    /// Are any of the contained values `NaN`?
    pub const fn is_nan(&self) -> bool
    where
        T: [const] IsNaN,
    {
        self.r0.is_nan() || self.r1.is_nan() || self.r2.is_nan()
    }
}

const impl<T> IsNaN for Matrix3x3<T>
where
    T: [const] IsNaN,
{
    fn is_nan(&self) -> bool {
        self.is_nan()
    }
}
