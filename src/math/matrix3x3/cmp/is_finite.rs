use crate::math::{Matrix3x3, number::IsFinite};

impl<T> Matrix3x3<T> {
    /// Are all the contained values finite?
    pub const fn is_finite(&self) -> bool
    where
        T: [const] IsFinite,
    {
        self.r0.is_finite() && self.r1.is_finite() && self.r2.is_finite()
    }
}
