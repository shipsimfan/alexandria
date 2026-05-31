use crate::math::{Vector3, number::Max};
use std::marker::Destruct;

impl<T> Vector3<T> {
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
        self.max_v(Vector3::splat(max))
    }
}
