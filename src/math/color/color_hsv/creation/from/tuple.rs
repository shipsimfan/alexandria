use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`] from a tuple
    pub const fn from_tuple((h, s, v): (T, T, T)) -> ColorHsv<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsv::new(h, s, v)
    }
}

const impl<T: [const] Destruct, Space: ColorSpace<T>> From<(T, T, T)> for ColorHsv<T, Space> {
    fn from(tuple: (T, T, T)) -> Self {
        ColorHsv::from_tuple(tuple)
    }
}
