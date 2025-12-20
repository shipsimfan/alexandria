use crate::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Convert this [`Color4`] into an array
    pub const fn into_array(self) -> [T; 4]
    where
        T: [const] Destruct,
    {
        [self.r, self.g, self.b, self.a]
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<[T; 4]> for Color4<T, Space> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
