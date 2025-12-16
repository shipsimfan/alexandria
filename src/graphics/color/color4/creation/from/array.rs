use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Create a new [`Color4`] from an array
    pub fn from_array([r, g, b, a]: [T; 4]) -> Self {
        Color4::new(r, g, b, a)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<[T; 4]> for Color4<T, Space> {
    fn from(array: [T; 4]) -> Self {
        Color4::from_array(array)
    }
}
