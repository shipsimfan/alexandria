use crate::math::{ColorHsva, ColorSpace};

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] from a slice
    pub const fn from_slice(slice: &[T]) -> ColorHsva<T, Space>
    where
        T: [const] Clone,
    {
        assert!(slice.len() >= 4);
        ColorHsva::new(
            slice[0].clone(),
            slice[1].clone(),
            slice[2].clone(),
            slice[3].clone(),
        )
    }
}

const impl<T: [const] Clone, Space: ColorSpace<T>> From<&[T]> for ColorHsva<T, Space> {
    fn from(slice: &[T]) -> Self {
        ColorHsva::from_slice(slice)
    }
}

const impl<T: [const] Clone, Space: ColorSpace<T>> From<&[T; 3]> for ColorHsva<T, Space> {
    fn from(slice: &[T; 3]) -> Self {
        ColorHsva::from_slice(slice)
    }
}
