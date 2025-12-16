use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color3<T, Space> {
    /// Convert this [`Color3`] into a tuple
    pub fn into_tuple(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Into<(T, T, T)> for Color3<T, Space> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}
