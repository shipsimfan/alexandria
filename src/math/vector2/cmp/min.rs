use crate::math::{Vector2, number::Min};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min_v(self, min: Vector2<T>) -> Vector2<T>
    where
        T: [const] Min + [const] Destruct,
    {
        self.zip(min, Min::min)
    }

    /// Get the component-wise minimum between this and `min`
    pub const fn min(self, min: T) -> Vector2<T>
    where
        T: [const] Clone + [const] Min + [const] Destruct,
    {
        self.min_v(Vector2::splat(min))
    }
}
