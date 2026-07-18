use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Convert this [`ColorHsva`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T, T)
    where
        T: [const] Destruct,
    {
        (self.h, self.s, self.v, self.a)
    }
}

const impl<T: [const] Destruct, Space: ColorSpace<T>> Into<(T, T, T, T)> for ColorHsva<T, Space> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
