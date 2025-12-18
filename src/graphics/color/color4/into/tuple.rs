use crate::graphics::color::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Convert this [`Color4`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T, T)
    where
        T: [const] Destruct,
    {
        (self.r, self.g, self.b, self.a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<(T, T, T, T)> for Color4<T, Space> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
