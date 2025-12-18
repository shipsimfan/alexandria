use crate::graphics::color::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`] from a tuple
    pub const fn from_tuple((r, g, b): (T, T, T)) -> Self
    where
        T: [const] Destruct,
    {
        Color3::new(r, g, b)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<(T, T, T)> for Color3<T, Space> {
    fn from(tuple: (T, T, T)) -> Self {
        Color3::from_tuple(tuple)
    }
}
