use crate::math::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Convert this color into a different color space
    pub const fn convert<Space2>(self) -> Color3<T, Space2>
    where
        Space: [const] ColorSpace<T>,
        Space2: [const] ColorSpace<T>,
    {
        Space2::from_linear(self.into_linear())
    }
}
