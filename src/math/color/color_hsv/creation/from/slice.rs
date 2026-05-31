use crate::math::{ColorHsv, ColorSpace};

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`] from a slice
    pub const fn from_slice(slice: &[T]) -> ColorHsv<T, Space>
    where
        T: [const] Clone,
    {
        assert!(slice.len() >= 3);
        ColorHsv::new(slice[0].clone(), slice[1].clone(), slice[2].clone())
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T]> for ColorHsv<T, Space> {
    fn from(slice: &[T]) -> Self {
        ColorHsv::from_slice(slice)
    }
}

impl<T: [const] Clone, Space: ColorSpace<T>> const From<&[T; 3]> for ColorHsv<T, Space> {
    fn from(slice: &[T; 3]) -> Self {
        ColorHsv::from_slice(slice)
    }
}
