use crate::math::Vector3;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`] from a slice
    pub const fn from_slice(s: &[T]) -> Self
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 3);
        Vector3::new(s[0].clone(), s[1].clone(), s[2].clone())
    }
}

impl<T: [const] Clone> const From<&[T]> for Vector3<T> {
    fn from(slice: &[T]) -> Self {
        Vector3::from_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[T; 3]> for Vector3<T> {
    fn from(slice: &[T; 3]) -> Self {
        Vector3::from_slice(slice)
    }
}
