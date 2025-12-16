use crate::graphics::color::{Color4, ColorSpace};

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Create a new [`Color4`] from a slice
    pub fn from_slice(slice: &[T]) -> Self {
        assert!(slice.len() >= 4);
        Color4::new(
            slice[0].clone(),
            slice[1].clone(),
            slice[2].clone(),
            slice[3].clone(),
        )
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<&[T]> for Color4<T, Space> {
    fn from(slice: &[T]) -> Self {
        Color4::from_slice(slice)
    }
}

impl<T: Clone, Space: ColorSpace<T> + ?Sized> From<&[T; 4]> for Color4<T, Space> {
    fn from(slice: &[T; 4]) -> Self {
        Color4::from_slice(slice)
    }
}
