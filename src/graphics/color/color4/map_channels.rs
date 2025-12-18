use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Map the red channel to a new value
    pub fn map_r<F: FnOnce(T) -> T>(mut self, f: F) -> Self {
        self.r = f(self.r);
        self
    }

    /// Map the green channel to a new value
    pub fn map_g<F: FnOnce(T) -> T>(mut self, f: F) -> Self {
        self.g = f(self.g);
        self
    }

    /// Map the blue channel to a new value
    pub fn map_b<F: FnOnce(T) -> T>(mut self, f: F) -> Self {
        self.b = f(self.b);
        self
    }

    /// Map the alpha channel to a new value
    pub fn map_a<F: FnOnce(T) -> T>(mut self, f: F) -> Self {
        self.a = f(self.a);
        self
    }

    /// Convert the channels of a color
    pub fn map_rgb<F: FnMut(T) -> T>(self, mut f: F) -> Color4<T, Space> {
        Color4::new(f(self.r), f(self.g), f(self.b), self.a)
    }

    /// Convert the channels of a color
    pub fn map_rgb_and_space<F: FnMut(T) -> T, Space2: ColorSpace<T>>(
        self,
        mut f: F,
    ) -> Color4<T, Space2> {
        Color4::new(f(self.r), f(self.g), f(self.b), self.a)
    }

    /// Convert the channels of a color
    pub fn map_channels<U, F: FnMut(T) -> U>(self, mut f: F) -> Color4<U, Space>
    where
        Space: ColorSpace<U>,
    {
        Color4::new(f(self.r), f(self.g), f(self.b), f(self.a))
    }

    /// Convert the channels of a color and change its space
    pub fn map_channels_and_space<U, Space2: ColorSpace<U>, F: FnMut(T) -> U>(
        self,
        mut f: F,
    ) -> Color4<U, Space2> {
        Color4::new(f(self.r), f(self.g), f(self.b), f(self.a))
    }

    /// Convert the channels of a color
    pub fn map_rgb_a<U, FC: FnMut(T) -> U, FA: FnOnce(T) -> U>(
        self,
        mut color: FC,
        alpha: FA,
    ) -> Color4<U, Space>
    where
        Space: ColorSpace<U>,
    {
        Color4::new(color(self.r), color(self.g), color(self.b), alpha(self.a))
    }

    /// Convert the channels of a color
    pub fn map_rgb_a_and_space<U, Space2: ColorSpace<U>, FC: FnMut(T) -> U, FA: FnOnce(T) -> U>(
        self,
        mut color: FC,
        alpha: FA,
    ) -> Color4<U, Space2> {
        Color4::new(color(self.r), color(self.g), color(self.b), alpha(self.a))
    }
}
