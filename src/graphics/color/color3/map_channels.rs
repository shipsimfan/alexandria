use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
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

    /// Convert the channels of a color
    pub fn map_channels<U, F: FnMut(T) -> U>(self, mut f: F) -> Color3<U, Space>
    where
        Space: ColorSpace<U>,
    {
        Color3::new(f(self.r), f(self.g), f(self.b))
    }

    /// Convert the channels of a color and change its space
    pub fn map_channels_and_space<U, Space2: ColorSpace<U>, F: FnMut(T) -> U>(
        self,
        mut f: F,
    ) -> Color3<U, Space2> {
        Color3::new(f(self.r), f(self.g), f(self.b))
    }
}
