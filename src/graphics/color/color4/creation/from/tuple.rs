use crate::graphics::color::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Create a new [`Color4`] from a tuple
    pub fn from_tuple((r, g, b, a): (T, T, T, T)) -> Self {
        Color4::new(r, g, b, a)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<(T, T, T, T)> for Color4<T, Space> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Color4::from_tuple(tuple)
    }
}
