use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color3<T, Space> {
    /// Create a new [`Color3`] from a tuple
    pub fn from_tuple((r, g, b): (T, T, T)) -> Self {
        Color3::new(r, g, b)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<(T, T, T)> for Color3<T, Space> {
    fn from(tuple: (T, T, T)) -> Self {
        Color3::from_tuple(tuple)
    }
}
