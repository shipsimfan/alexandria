use crate::{Color3, ColorSpace};
use alexandria_math::number::Clamp;
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Clamp this color channel wise between `min` and `max`
    pub const fn clamp(self, min: T::Bound, max: T::Bound) -> Color3<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        self.clamp_c(Color3::gray(min), Color3::gray(max))
    }

    /// Clamp this color channel-wise between `min` and `max`
    pub const fn clamp_c(
        self,
        min: Color3<T::Bound, Space>,
        max: Color3<T::Bound, Space>,
    ) -> Color3<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        Color3::new(
            self.r.clamp(min.r, max.r),
            self.g.clamp(min.g, max.g),
            self.b.clamp(min.b, max.b),
        )
    }
}

impl<T: [const] Clamp + [const] Destruct, Space: ColorSpace<T>> const Clamp for Color3<T, Space>
where
    T: [const] Clamp + [const] Destruct,
    T::Bound: [const] Clone + [const] Destruct,
    Space: ColorSpace<T::Bound>,
{
    type Bound = T::Bound;

    fn clamp(self, min: Self::Bound, max: Self::Bound) -> Self {
        self.clamp(min, max)
    }
}
