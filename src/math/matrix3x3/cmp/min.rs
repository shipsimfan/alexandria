use crate::math::{Matrix3x3, number::Min};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min_m(self, min: Self) -> Self
    where
        T: [const] Min + [const] Destruct,
    {
        self.zip(min, Min::min)
    }

    /// Get the component-wise minimum between this and `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Clone + [const] Min + [const] Destruct,
    {
        self.min_m(Matrix3x3::splat(min))
    }
}

impl<T> const Min for Matrix3x3<T>
where
    T: [const] Min + [const] Destruct,
{
    fn min(self, other: Self) -> Self {
        self.min_m(other)
    }
}
