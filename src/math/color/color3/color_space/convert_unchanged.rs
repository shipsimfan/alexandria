use crate::math::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Convert this color into a different color space without changing the values
    pub const unsafe fn convert_unchanged<Space2: ColorSpace<T>>(self) -> Color3<T, Space2>
    where
        T: [const] Destruct,
    {
        Color3::new(self.r, self.g, self.b)
    }
}
