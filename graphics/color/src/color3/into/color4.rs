use crate::{Color3, Color4, ColorSpace};
use alexandria_math::number::One;
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color4`] with this color and an alpha of `a`
    pub const fn with_alpha(self, a: T) -> Color4<T, Space>
    where
        T: [const] Destruct,
    {
        Color4::new(self.r, self.g, self.b, a)
    }
}

impl<T: [const] Destruct + One, Space: ColorSpace<T>> const Into<Color4<T, Space>>
    for Color3<T, Space>
{
    fn into(self) -> Color4<T, Space> {
        self.with_alpha(T::NORMALIZED_ONE)
    }
}
