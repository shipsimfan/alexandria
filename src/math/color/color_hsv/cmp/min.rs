use crate::math::{ColorHsv, ColorSpace, number::Min};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Set all channels to be at most `min`
    pub const fn min(self, min: T) -> ColorHsv<T, Space>
    where
        T: [const] Min + [const] Clone + [const] Destruct,
    {
        self.min_c(ColorHsv::new(min.clone(), min.clone(), min))
    }

    /// Set all channels to be at most `min` channel-wise
    pub const fn min_c(self, min: ColorHsv<T, Space>) -> ColorHsv<T, Space>
    where
        T: [const] Min + [const] Destruct,
    {
        ColorHsv::new(self.h.min(min.h), self.s.min(min.s), self.v.min(min.v))
    }
}

impl<T: [const] Min + [const] Destruct, Space: ColorSpace<T>> const Min for ColorHsv<T, Space> {
    fn min(self, other: Self) -> Self {
        self.min_c(other)
    }
}
