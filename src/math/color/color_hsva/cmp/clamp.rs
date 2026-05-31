use crate::math::{ColorHsva, ColorSpace, number::Clamp};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Clamp this color channel wise between `min` and `max`
    pub const fn clamp(self, min: T::Bound, max: T::Bound) -> ColorHsva<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        self.clamp_c(ColorHsva::splat(min), ColorHsva::splat(max))
    }

    /// Clamp this color channel-wise between `min` and `max`
    pub const fn clamp_c(
        self,
        min: ColorHsva<T::Bound, Space>,
        max: ColorHsva<T::Bound, Space>,
    ) -> ColorHsva<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        ColorHsva::new(
            self.h.clamp(min.h, max.h),
            self.s.clamp(min.s, max.s),
            self.v.clamp(min.v, max.v),
            self.a.clamp(min.a, max.a),
        )
    }
}

impl<T: [const] Clamp + [const] Destruct, Space: ColorSpace<T>> const Clamp for ColorHsva<T, Space>
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
