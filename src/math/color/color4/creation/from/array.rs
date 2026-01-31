use crate::math::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] from an array
    pub const fn from_array([r, g, b, a]: [T; 4]) -> Color4<T, Space>
    where
        T: [const] Destruct,
    {
        Color4::new(r, g, b, a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<[T; 4]> for Color4<T, Space> {
    fn from(array: [T; 4]) -> Self {
        Color4::from_array(array)
    }
}
