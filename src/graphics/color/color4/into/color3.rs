use crate::graphics::color::{Color3, Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Drop the alpha channel from this color
    pub const fn rgb(self) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        Color3::new(self.r, self.g, self.b)
    }

    /// Split this [`Color4`] into its color and alpha portions
    pub const fn rgb_a(self) -> (Color3<T, Space>, T)
    where
        T: [const] Destruct,
    {
        (Color3::new(self.r, self.g, self.b), self.a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<Color3<T, Space>> for Color4<T, Space> {
    fn into(self) -> Color3<T, Space> {
        self.rgb()
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<(Color3<T, Space>, T)>
    for Color4<T, Space>
{
    fn into(self) -> (Color3<T, Space>, T) {
        self.rgb_a()
    }
}
