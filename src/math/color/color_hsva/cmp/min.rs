use crate::math::{ColorHsva, ColorSpace, number::Min};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> ColorHsva<T, Space>
    where
        T: [const] Min + [const] Clone + [const] Destruct,
    {
        self.min_c(ColorHsva::new(min.clone(), min.clone(), min.clone(), min))
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_c(self, min: ColorHsva<T, Space>) -> ColorHsva<T, Space>
    where
        T: [const] Min + [const] Destruct,
    {
        ColorHsva::new(
            self.h.min(min.h),
            self.s.min(min.s),
            self.v.min(min.v),
            self.a.min(min.a),
        )
    }
}

impl<T: [const] Min + [const] Destruct, Space: ColorSpace<T>> const Min for ColorHsva<T, Space> {
    fn min(self, other: Self) -> Self {
        self.min_c(other)
    }
}
