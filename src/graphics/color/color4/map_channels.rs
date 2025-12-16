use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
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
}
