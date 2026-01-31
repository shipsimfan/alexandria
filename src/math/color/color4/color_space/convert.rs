use crate::math::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Convert this color into a different color space
    pub const fn convert<Space2>(self) -> Color4<T, Space2>
    where
        T: [const] Destruct,
        Space: [const] ColorSpace<T>,
        Space2: [const] ColorSpace<T>,
    {
        let (color, a) = self.rgb_a();
        color.convert().with_alpha(a)
    }
}
