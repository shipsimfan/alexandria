use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T> + ?Sized> Color3<T, Space> {
    /// Create a new [`Color3`] from an array
    pub fn from_array([r, g, b]: [T; 3]) -> Self {
        Color3::new(r, g, b)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<[T; 3]> for Color3<T, Space> {
    fn from(array: [T; 3]) -> Self {
        Color3::from_array(array)
    }
}
