use crate::math::Vector2;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] from a slice
    pub const fn from_slice(s: &[T]) -> Vector2<T>
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 2);
        Vector2::new(s[0].clone(), s[1].clone())
    }
}

impl<T: [const] Clone> const From<&[T]> for Vector2<T> {
    fn from(slice: &[T]) -> Self {
        Vector2::from_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[T; 2]> for Vector2<T> {
    fn from(slice: &[T; 2]) -> Self {
        Vector2::from_slice(slice)
    }
}
