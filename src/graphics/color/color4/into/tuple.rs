use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Convert this [`Color4`] into a tuple
    pub fn into_tuple(self) -> (T, T, T, T) {
        (self.r, self.g, self.b, self.a)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Into<(T, T, T, T)> for Color4<T, Space> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
