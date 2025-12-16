use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Convert this [`Color4`] into an array
    pub fn into_array(self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Into<[T; 4]> for Color4<T, Space> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
