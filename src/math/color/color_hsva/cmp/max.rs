use crate::math::{ColorHsva, ColorSpace, number::Max};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> ColorHsva<T, Space>
    where
        T: [const] Max + [const] Clone + [const] Destruct,
    {
        self.max_c(ColorHsva::new(max.clone(), max.clone(), max.clone(), max))
    }

    /// Set all channels to be at least `max` channel-wise
    pub const fn max_c(self, max: ColorHsva<T, Space>) -> ColorHsva<T, Space>
    where
        T: [const] Max + [const] Destruct,
    {
        ColorHsva::new(
            self.h.max(max.h),
            self.s.max(max.s),
            self.v.max(max.v),
            self.a.max(max.a),
        )
    }
}

impl<T: [const] Max + [const] Destruct, Space: ColorSpace<T>> const Max for ColorHsva<T, Space> {
    fn max(self, other: Self) -> Self {
        self.max_c(other)
    }
}
