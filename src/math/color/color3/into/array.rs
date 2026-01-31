use crate::math::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Convert this [`Color3`] into an array
    pub const fn into_array(self) -> [T; 3]
    where
        T: [const] Destruct,
    {
        [self.r, self.g, self.b]
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<[T; 3]> for Color3<T, Space> {
    fn into(self) -> [T; 3] {
        self.into_array()
    }
}
