use crate::graphics::color::{Color3, ColorSpace};

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Color3<T, Space> {
    /// Create a new [`Color3`] from a slice
    pub fn from_slice(slice: &[T]) -> Self {
        assert!(slice.len() >= 3);
        Color3::new(slice[0].clone(), slice[1].clone(), slice[2].clone())
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<&[T]> for Color3<T, Space> {
    fn from(slice: &[T]) -> Self {
        Color3::from_slice(slice)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<&[T; 3]> for Color3<T, Space> {
    fn from(slice: &[T; 3]) -> Self {
        Color3::from_slice(slice)
    }
}
