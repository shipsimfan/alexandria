use crate::math::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] from a slice
    pub const fn from_slice(slice: &[T]) -> Color4<T, Space>
    where
        T: [const] Clone,
    {
        assert!(slice.len() >= 4);
        Color4::new(
            slice[0].clone(),
            slice[1].clone(),
            slice[2].clone(),
            slice[3].clone(),
        )
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T]> for Color4<T, Space> {
    fn from(slice: &[T]) -> Self {
        Color4::from_slice(slice)
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T; 4]> for Color4<T, Space> {
    fn from(slice: &[T; 4]) -> Self {
        Color4::from_slice(slice)
    }
}
