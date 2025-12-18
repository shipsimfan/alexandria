use crate::graphics::color::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] from a tuple
    pub const fn from_tuple((r, g, b, a): (T, T, T, T)) -> Self
    where
        T: [const] Destruct,
    {
        Color4::new(r, g, b, a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<(T, T, T, T)> for Color4<T, Space> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Color4::from_tuple(tuple)
    }
}
