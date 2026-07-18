use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Convert this [`ColorHsv`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T)
    where
        T: [const] Destruct,
    {
        (self.h, self.s, self.v)
    }
}

const impl<T: [const] Destruct, Space: ColorSpace<T>> Into<(T, T, T)> for ColorHsv<T, Space> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}
