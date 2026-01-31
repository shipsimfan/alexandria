use crate::math::{Color3, ColorSpace, Linear};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Convert this covert this color from its current color space into [`Linear`] color space
    pub const fn into_linear(self) -> Color3<T, Linear>
    where
        Space: [const] ColorSpace<T>,
    {
        Space::into_linear(self)
    }
}
