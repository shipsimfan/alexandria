use crate::math::{ColorHsv, ColorSpace, number::Clamp};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Clamp this color channel wise between `min` and `max`
    pub const fn clamp(self, min: T::Bound, max: T::Bound) -> ColorHsv<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        self.clamp_c(ColorHsv::gray(min), ColorHsv::gray(max))
    }

    /// Clamp this color channel-wise between `min` and `max`
    pub const fn clamp_c(
        self,
        min: ColorHsv<T::Bound, Space>,
        max: ColorHsv<T::Bound, Space>,
    ) -> ColorHsv<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Destruct,
        Space: ColorSpace<T::Bound>,
    {
        ColorHsv::new(
            self.h.clamp(min.h, max.h),
            self.s.clamp(min.s, max.s),
            self.v.clamp(min.v, max.v),
        )
    }
}

const impl<T: [const] Clamp + [const] Destruct, Space: ColorSpace<T>> Clamp for ColorHsv<T, Space>
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
