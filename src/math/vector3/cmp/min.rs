use crate::math::{Vector3, number::Min};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min_v(self, min: Self) -> Self
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
        self.min_v(Vector3::splat(min))
    }
}

impl<T> const Min for Vector3<T>
where
    T: [const] Min + [const] Destruct,
{
    fn min(self, other: Self) -> Self {
        self.min_v(other)
    }
}
