use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color3<T, Space> {
    /// Convert this [`Color3`] into an array
    pub fn into_array(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Into<[T; 3]> for Color3<T, Space> {
    fn into(self) -> [T; 3] {
        self.into_array()
    }
}
