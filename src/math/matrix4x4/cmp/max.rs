use crate::math::{Matrix4x4, number::Max};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Get the component-wise maximum between this and `max`
    pub const fn max(self, max: T) -> Self
    where
        T: [const] Clone + [const] Max + [const] Destruct,
    {
        self.max_m(Matrix4x4::splat(max))
    }

    /// Get the component-wise maximum between this and `max`
    pub const fn max_m(self, max: Self) -> Self
    where
        T: [const] Max + [const] Destruct,
    {
        self.zip(max, Max::max)
    }
}

impl<T> const Max for Matrix4x4<T>
where
    T: [const] Max + [const] Destruct,
{
    fn max(self, other: Self) -> Self {
        self.max_m(other)
    }
}
