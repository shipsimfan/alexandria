use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Convert this [`ColorHsva`] into an array
    pub const fn into_array(self) -> [T; 4]
    where
        T: [const] Destruct,
    {
        [self.h, self.s, self.v, self.a]
    }
}

const impl<T: [const] Destruct, Space: ColorSpace<T>> Into<[T; 4]> for ColorHsva<T, Space> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
