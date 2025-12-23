use crate::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`] from an array
    pub const fn from_array([r, g, b]: [T; 3]) -> Color3<T, Space>
    where
        T: [const] Destruct,
    {
        Color3::new(r, g, b)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<[T; 3]> for Color3<T, Space> {
    fn from(array: [T; 3]) -> Self {
        Color3::from_array(array)
    }
}
