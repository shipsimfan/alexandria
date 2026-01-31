use crate::math::{Matrix4x4, number::Clamp};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Clamp this vector component-wise between `min` and `max`
    pub const fn clamp_m(self, min: Matrix4x4<T::Bound>, max: Matrix4x4<T::Bound>) -> Self
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Destruct,
    {
        self.zip2(min, max, Clamp::clamp)
    }

    /// Clamp this vector component-wise between `min` and `max`
    pub const fn clamp(self, min: T::Bound, max: T::Bound) -> Self
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct,
    {
        self.clamp_m(Matrix4x4::splat(min), Matrix4x4::splat(max))
    }
}

impl<T: [const] Clamp + [const] Destruct> const Clamp for Matrix4x4<T>
where
    T::Bound: [const] Clone + [const] Destruct,
{
    type Bound = T::Bound;

    fn clamp(self, min: Self::Bound, max: Self::Bound) -> Self {
        self.clamp(min, max)
    }
}
