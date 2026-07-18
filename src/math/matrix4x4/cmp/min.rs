use crate::math::{Matrix4x4, number::Min};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Clone + [const] Min + [const] Destruct,
    {
        self.min_m(Matrix4x4::splat(min))
    }

    /// Get the component-wise minimum between this and `min`
    pub const fn min_m(self, min: Self) -> Self
    where
        T: [const] Min + [const] Destruct,
    {
        self.zip(min, Min::min)
    }
}

const impl<T> Min for Matrix4x4<T>
where
    T: [const] Min + [const] Destruct,
{
    fn min(self, other: Self) -> Self {
        self.min_m(other)
    }
}
