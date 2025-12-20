use crate::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Map the red channel to a new value
    pub const fn map_r<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        self.r = f(self.r);
        self
    }

    /// Map the green channel to a new value
    pub const fn map_g<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        self.g = f(self.g);
        self
    }

    /// Map the blue channel to a new value
    pub const fn map_b<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        self.b = f(self.b);
        self
    }

    /// Map the alpha channel to a new value
    pub const fn map_a<F: [const] FnOnce(T) -> T>(mut self, f: F) -> Self
    where
        T: [const] Destruct,
    {
        self.a = f(self.a);
        self
    }

    /// Convert only the RGB channels of a color
    pub const fn map_rgb<F: [const] FnMut(T) -> T + [const] Destruct>(
        self,
        mut f: F,
    ) -> Color4<T, Space>
    where
        T: [const] Destruct,
    {
        Color4::new(f(self.r), f(self.g), f(self.b), self.a)
    }

    /// Convert only the RGB channels of a color and change its space
    pub const unsafe fn map_rgb_and_space<F: [const] FnMut(T) -> T + [const] Destruct>(
        self,
        mut f: F,
    ) -> Color4<T, Space>
    where
        T: [const] Destruct,
    {
        Color4::new(f(self.r), f(self.g), f(self.b), self.a)
    }

    /// Convert the RGB channels and alpha channels of a color using different functions
    pub const fn map_rgb_a<
        U,
        FColor: [const] FnMut(T) -> U + [const] Destruct,
        FAlpha: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut fc: FColor,
        mut fa: FAlpha,
    ) -> Color4<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        Color4::new(fc(self.r), fc(self.g), fc(self.b), fa(self.a))
    }

    /// Convert the RGB channels and alpha channels of a color using different functions and change
    /// its space
    pub const unsafe fn map_rgb_a_and_space<
        U,
        Space2: ColorSpace<U>,
        FColor: [const] FnMut(T) -> U + [const] Destruct,
        FAlpha: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut fc: FColor,
        mut fa: FAlpha,
    ) -> Color4<U, Space2>
    where
        T: [const] Destruct,
    {
        Color4::new(fc(self.r), fc(self.g), fc(self.b), fa(self.a))
    }

    /// Convert the channels of a color
    pub const fn map_channels<U, F: [const] FnMut(T) -> U + [const] Destruct>(
        self,
        mut f: F,
    ) -> Color4<U, Space>
    where
        Space: ColorSpace<U>,
        T: [const] Destruct,
    {
        Color4::new(f(self.r), f(self.g), f(self.b), f(self.a))
    }

    /// Convert the channels of a color and change its space
    pub const unsafe fn map_channels_and_space<
        U,
        Space2: ColorSpace<U>,
        F: [const] FnMut(T) -> U + [const] Destruct,
    >(
        self,
        mut f: F,
    ) -> Color4<U, Space2>
    where
        T: [const] Destruct,
    {
        Color4::new(f(self.r), f(self.g), f(self.b), f(self.a))
    }
}
