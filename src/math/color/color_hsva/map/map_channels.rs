use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Map the hue channel to a new value
    pub const fn map_h<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        self.h = f(self.h);
        self
    }

    /// Map the saturation channel to a new value
    pub const fn map_s<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        self.s = f(self.s);
        self
    }

    /// Map the value channel to a new value
    pub const fn map_v<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        self.v = f(self.v);
        self
    }

    /// Map the alpha channel to a new value
    pub const fn map_a<F: [const] FnOnce(T) -> T>(mut self, f: F) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        self.a = f(self.a);
        self
    }

    /// Convert only the HSV channels of a color
    pub const fn map_hsv<F: [const] FnMut(T) -> T + [const] Destruct>(
        self,
        mut f: F,
    ) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(f(self.h), f(self.s), f(self.v), self.a)
    }

    /// Convert only the HSV channels of a color and change its space
    pub const unsafe fn map_hsv_and_space<
        Space2: ColorSpace<T>,
        F: [const] FnMut(T) -> T + [const] Destruct,
    >(
        self,
        mut f: F,
    ) -> ColorHsva<T, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(f(self.h), f(self.s), f(self.v), self.a)
    }

    /// Convert the HSV channels and alpha channels of a color using different functions
    pub const fn map_hsv_a<
        U,
        FColor: [const] FnMut(T) -> U + [const] Destruct,
        FAlpha: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut fc: FColor,
        mut fa: FAlpha,
    ) -> ColorHsva<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        ColorHsva::new(fc(self.h), fc(self.s), fc(self.v), fa(self.a))
    }

    /// Convert the HSV channels and alpha channels of a color using different functions and change
    /// its space
    pub const unsafe fn map_hsv_a_and_space<
        U,
        Space2: ColorSpace<U>,
        FColor: [const] FnMut(T) -> U + [const] Destruct,
        FAlpha: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut fc: FColor,
        mut fa: FAlpha,
    ) -> ColorHsva<U, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(fc(self.h), fc(self.s), fc(self.v), fa(self.a))
    }

    /// Convert the channels of a color
    pub const fn map_channels<U, F: [const] FnMut(T) -> U + [const] Destruct>(
        self,
        mut f: F,
    ) -> ColorHsva<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        ColorHsva::new(f(self.h), f(self.s), f(self.v), f(self.a))
    }

    /// Convert the channels of a color and change its space
    pub const unsafe fn map_channels_and_space<
        U,
        Space2: ColorSpace<U>,
        F: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut f: F,
    ) -> ColorHsva<U, Space2>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(f(self.h), f(self.s), f(self.v), f(self.a))
    }
}
