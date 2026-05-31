use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Convert this [`ColorHsv`] into an array
    pub const fn into_array(self) -> [T; 3]
    where
        T: [const] Destruct,
    {
        [self.h, self.s, self.v]
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<[T; 3]> for ColorHsv<T, Space> {
    fn into(self) -> [T; 3] {
        self.into_array()
    }
}
