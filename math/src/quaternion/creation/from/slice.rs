use crate::Quaternion;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`] from a slice
    pub const fn from_slice(s: &[T]) -> Self
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 4);
        Quaternion::new(s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone())
    }
}

impl<T: [const] Clone> const From<&[T]> for Quaternion<T> {
    fn from(slice: &[T]) -> Self {
        Quaternion::from_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[T; 4]> for Quaternion<T> {
    fn from(slice: &[T; 4]) -> Self {
        Quaternion::from_slice(slice)
    }
}
