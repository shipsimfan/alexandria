use crate::graphics::color::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Change the contained red channel value
    pub const fn with_r(self, r: T) -> Self
    where
        T: [const] Destruct,
    {
        Color4::new(r, self.g, self.b, self.a)
    }

    /// Change the contained green channel value
    pub const fn with_g(self, g: T) -> Self
    where
        T: [const] Destruct,
    {
        Color4::new(self.r, g, self.b, self.a)
    }

    /// Change the contained blue channel value
    pub const fn with_b(self, b: T) -> Self
    where
        T: [const] Destruct,
    {
        Color4::new(self.r, self.g, b, self.a)
    }

    /// Change the contained alpha channel value
    pub const fn with_a(self, a: T) -> Self
    where
        T: [const] Destruct,
    {
        Color4::new(self.r, self.g, self.b, a)
    }
}
