use crate::Matrix3x3;
use std::marker::Destruct;

const fn component_max<T: [const] PartialOrd + [const] Destruct>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

impl<T> Matrix3x3<T> {
    /// Get the component-wise maximum between this and `max`
    pub const fn max(self, max: T) -> Self
    where
        T: [const] Clone + [const] PartialOrd + [const] Destruct,
    {
        self.max_m(Matrix3x3::splat(max))
    }

    /// Get the component-wise maximum between this and `max`
    pub const fn max_m(self, max: Self) -> Self
    where
        T: [const] PartialOrd + [const] Destruct,
    {
        self.zip(max, component_max)
    }
}
