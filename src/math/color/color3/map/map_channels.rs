use crate::math::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Map the red channel to a new value
    pub const fn map_r<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        self.r = f(self.r);
        self
    }

    /// Map the green channel to a new value
    pub const fn map_g<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        self.g = f(self.g);
        self
    }

    /// Map the blue channel to a new value
    pub const fn map_b<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        self.b = f(self.b);
        self
    }

    /// Convert the channels of a color
    pub const fn map_channels<U, F: [const] FnMut(T) -> U + [const] Destruct>(
        self,
        mut f: F,
    ) -> Color3<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        Color3::new(f(self.r), f(self.g), f(self.b))
    }

    /// Convert the channels of a color and change its space
    pub const unsafe fn map_channels_and_space<
        U,
        Space2: ColorSpace<U>,
        F: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut f: F,
    ) -> Color3<U, Space2>
    where
        T: [const] Destruct,
    {
        Color3::new(f(self.r), f(self.g), f(self.b))
    }
}
