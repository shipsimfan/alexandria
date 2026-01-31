use crate::math::Matrix4x4;
use std::marker::Destruct;

const fn component_min<T: [const] PartialOrd + [const] Destruct>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

impl<T> Matrix4x4<T> {
    /// Get the component-wise minimum between this and `min`
    pub const fn min(self, min: T) -> Self
    where
        T: [const] Clone + [const] PartialOrd + [const] Destruct,
    {
        self.min_m(Matrix4x4::splat(min))
    }

    /// Get the component-wise minimum between this and `min`
    pub const fn min_m(self, min: Self) -> Self
    where
        T: [const] PartialOrd + [const] Destruct,
    {
        self.zip(min, component_min)
    }
}
