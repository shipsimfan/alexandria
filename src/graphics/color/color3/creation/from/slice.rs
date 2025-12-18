use crate::graphics::color::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`] from a slice
    pub const fn from_slice(slice: &[T]) -> Self
    where
        T: [const] Clone,
    {
        assert!(slice.len() >= 3);
        Color3::new(slice[0].clone(), slice[1].clone(), slice[2].clone())
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T]> for Color3<T, Space> {
    fn from(slice: &[T]) -> Self {
        Color3::from_slice(slice)
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T; 3]> for Color3<T, Space> {
    fn from(slice: &[T; 3]) -> Self {
        Color3::from_slice(slice)
    }
}
