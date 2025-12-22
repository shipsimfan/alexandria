use crate::Vector2;
use std::marker::Destruct;

const fn component_min<T: [const] PartialOrd + [const] Destruct>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

impl<T> Vector2<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min_v(self, min: Self) -> Self
    where
        T: [const] PartialOrd + [const] Destruct,
    {
        self.zip(min, component_min)
    }

    /// Get the component-wise minimum between this and `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Clone + [const] PartialOrd + [const] Destruct,
    {
        self.min_v(Vector2::splat(min))
    }
}
