use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
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
