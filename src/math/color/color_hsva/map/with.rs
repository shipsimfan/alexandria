use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Change the contained hue channel value
    pub const fn with_h(self, h: T) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(h, self.s, self.v, self.a)
    }

    /// Change the contained saturation channel value
    pub const fn with_s(self, s: T) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(self.h, s, self.v, self.a)
    }

    /// Change the contained value channel value
    pub const fn with_v(self, v: T) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(self.h, self.s, v, self.a)
    }

    /// Change the contained alpha channel value
    pub const fn with_a(self, a: T) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(self.h, self.s, self.v, a)
    }
}
