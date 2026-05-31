use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Map the hue channel to a new value
    pub const fn map_h<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        self.h = f(self.h);
        self
    }

    /// Map the saturation channel to a new value
    pub const fn map_s<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        self.s = f(self.s);
        self
    }

    /// Map the value channel to a new value
    pub const fn map_v<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        self.v = f(self.v);
        self
    }

    /// Convert the channels of a color
    pub const fn map_channels<U, F: [const] FnMut(T) -> U + [const] Destruct>(
        self,
        mut f: F,
    ) -> ColorHsv<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        ColorHsv::new(f(self.h), f(self.s), f(self.v))
    }

    /// Convert the channels of a color and change its space
    pub const unsafe fn map_channels_and_space<
        U,
        Space2: ColorSpace<U>,
        F: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut f: F,
    ) -> ColorHsv<U, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(f(self.h), f(self.s), f(self.v))
    }
}
