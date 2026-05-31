use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Change the contained hue channel value
    pub const fn with_h(self, h: T) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(h, self.s, self.v)
    }

    /// Change the contained saturation channel value
    pub const fn with_s(self, s: T) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(self.h, s, self.v)
    }

    /// Change the contained value channel value
    pub const fn with_v(self, v: T) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(self.h, self.s, v)
    }
}
