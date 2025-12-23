use crate::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Change the contained red channel value
    pub const fn with_r(self, r: T) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        Color3::new(r, self.g, self.b)
    }

    /// Change the contained green channel value
    pub const fn with_g(self, g: T) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        Color3::new(self.r, g, self.b)
    }

    /// Change the contained blue channel value
    pub const fn with_b(self, b: T) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        Color3::new(self.r, self.g, b)
    }
}
