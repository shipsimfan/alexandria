use crate::math::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Convert this color into a different color space without changing the values
    pub const unsafe fn convert_unchanged<Space2: ColorSpace<T>>(self) -> Color4<T, Space2>
    where
        T: [const] Destruct,
    {
        Color4::new(self.r, self.g, self.b, self.a)
    }
}
