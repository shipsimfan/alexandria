use crate::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Convert this [`Color3`] into a tuple
    pub const fn into_tuple(self) -> (T, T, T)
    where
        T: [const] Destruct,
    {
        (self.r, self.g, self.b)
    }
}

impl<T: [const] Destruct, Space: ColorSpace<T>> const Into<(T, T, T)> for Color3<T, Space> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}
