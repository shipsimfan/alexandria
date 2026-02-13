use crate::math::Rect;

impl<T> Rect<T> {
    /// Create a new [`Rect`] from a slice of values
    pub const fn from_slice(s: &[T]) -> Rect<T>
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 4);
        Rect::from_xywh(s[0].clone(), s[1].clone(), s[2].clone(), s[3].clone())
    }
}

impl<T: [const] Clone> const From<&[T]> for Rect<T> {
    fn from(slice: &[T]) -> Self {
        Rect::from_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[T; 4]> for Rect<T> {
    fn from(slice: &[T; 4]) -> Self {
        Rect::from_slice(slice)
    }
}
