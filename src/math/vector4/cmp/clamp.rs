use crate::math::{Vector4, number::Clamp};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Clamp this vector component-wise between `min` and `max`
    pub const fn clamp_v(self, min: Vector4<T::Bound>, max: Vector4<T::Bound>) -> Self
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
        self.clamp_v(Vector4::splat(min), Vector4::splat(max))
    }
}

impl<T: [const] Clamp + [const] Destruct> const Clamp for Vector4<T>
where
    T::Bound: [const] Clone + [const] Destruct,
{
    type Bound = T::Bound;

    fn clamp(self, min: Self::Bound, max: Self::Bound) -> Self {
        self.clamp(min, max)
    }
}
