use crate::math::{Vector4, number::Max};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Get the component-wise maximum between this and `max`
    pub const fn max_v(self, max: Self) -> Self
    where
        T: [const] Max + [const] Destruct,
    {
        self.zip(max, Max::max)
    }

    /// Get the component-wise maximum between this and `max`
    pub const fn max(self, max: T) -> Self
    where
        T: [const] Clone + [const] Max + [const] Destruct,
    {
        self.max_v(Vector4::splat(max))
    }
}

impl<T> const Max for Vector4<T>
where
    T: [const] Max + [const] Destruct,
{
    fn max(self, max: Self) -> Self {
        self.max_v(max)
    }
}
