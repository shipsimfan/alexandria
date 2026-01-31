use crate::math::Vector2;
use std::marker::Destruct;

const fn component_max<T: [const] PartialOrd + [const] Destruct>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

impl<T> Vector2<T> {
    /// Get the component-wise maximum between this and `max`
    pub const fn max_v(self, max: Vector2<T>) -> Vector2<T>
    where
        T: [const] PartialOrd + [const] Destruct,
    {
        self.zip(max, component_max)
    }

    /// Get the component-wise maximum between this and `max`
    pub const fn max(self, max: T) -> Vector2<T>
    where
        T: [const] Clone + [const] PartialOrd + [const] Destruct,
    {
        self.max_v(Vector2::splat(max))
    }
}
