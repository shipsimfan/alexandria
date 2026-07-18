use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`] from an array
    pub const fn from_array([h, s, v]: [T; 3]) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(h, s, v)
    }
}

const impl<T: [const] Destruct, Space: ColorSpace<T>> From<[T; 3]> for ColorHsv<T, Space> {
    fn from(array: [T; 3]) -> Self {
        ColorHsv::from_array(array)
    }
}
