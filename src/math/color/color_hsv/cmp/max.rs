use crate::math::{ColorHsv, ColorSpace, number::Max};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Set all channels to be at least `max`
    pub const fn max(self, max: T) -> ColorHsv<T, Space>
    where
        T: [const] Max + [const] Clone + [const] Destruct,
    {
        self.max_c(ColorHsv::new(max.clone(), max.clone(), max))
    }

    /// Set all channels to be at least `max` channel-wise
    pub const fn max_c(self, max: ColorHsv<T, Space>) -> ColorHsv<T, Space>
    where
        T: [const] Max + [const] Destruct,
    {
        ColorHsv::new(self.h.max(max.h), self.s.max(max.s), self.v.max(max.v))
    }
}

const impl<T: [const] Max + [const] Destruct, Space: ColorSpace<T>> Max for ColorHsv<T, Space> {
    fn max(self, other: Self) -> Self {
        self.max_c(other)
    }
}
