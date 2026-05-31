use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] from a tuple
    pub const fn from_tuple((h, s, v, a): (T, T, T, T)) -> ColorHsva<T, Space>
    where
        T: [const] Destruct,
    {
        ColorHsva::new(h, s, v, a)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const From<(T, T, T, T)> for ColorHsva<T, Space> {
    fn from(tuple: (T, T, T, T)) -> Self {
        ColorHsva::from_tuple(tuple)
    }
}
