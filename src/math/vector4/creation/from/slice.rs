use crate::math::Vector4;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`] from a slice
    pub const fn from_slice(s: &[T]) -> Self
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 4);
        Vector4::new(s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone())
    }
}

impl<T: [const] Clone> const From<&[T]> for Vector4<T> {
    fn from(slice: &[T]) -> Self {
        Vector4::from_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[T; 4]> for Vector4<T> {
    fn from(slice: &[T; 4]) -> Self {
        Vector4::from_slice(slice)
    }
}
