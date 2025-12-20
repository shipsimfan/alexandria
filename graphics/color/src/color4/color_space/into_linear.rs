use crate::{Color4, ColorSpace, Linear};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Convert this covert this color from its current color space into [`Linear`] color space
    pub const fn into_linear(self) -> Color4<T, Linear>
    where
        T: [const] Destruct,
        Space: [const] ColorSpace<T>,
    {
        let (color, a) = self.rgb_a();
        color.into_linear().with_alpha(a)
    }
}
