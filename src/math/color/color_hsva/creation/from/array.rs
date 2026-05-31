use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] from an array
    pub const fn from_array([h, s, v, a]: [T; 4]) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(h, s, v, a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<[T; 4]> for ColorHsva<T, Space> {
    fn from(array: [T; 4]) -> Self {
        ColorHsva::from_array(array)
    }
}
