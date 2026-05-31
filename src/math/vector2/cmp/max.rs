use crate::math::{Vector2, number::Max};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Get the component-wise maximum between this and `max`
    pub const fn max_v(self, max: Vector2<T>) -> Vector2<T>
    where
        T: [const] Max + [const] Destruct,
    {
        self.zip(max, Max::max)
    }

    /// Get the component-wise maximum between this and `max`
    pub const fn max(self, max: T) -> Vector2<T>
    where
        T: [const] Clone + [const] Max + [const] Destruct,
    {
        self.max_v(Vector2::splat(max))
    }
}
