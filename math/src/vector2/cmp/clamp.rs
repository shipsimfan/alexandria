use crate::{Vector2, number::Clamp};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Clamp this vector component-wise between `min` and `max`
    pub const fn clamp_v(self, min: Vector2<T::Bound>, max: Vector2<T::Bound>) -> Self
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
        self.clamp_v(Vector2::splat(min), Vector2::splat(max))
    }
}

impl<T: [const] Clamp + [const] Destruct> const Clamp for Vector2<T>
where
    T::Bound: [const] Clone + [const] Destruct,
{
    type Bound = T::Bound;

    fn clamp(self, min: Self::Bound, max: Self::Bound) -> Self {
        self.clamp(min, max)
    }
}
