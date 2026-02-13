use crate::math::{Rect, Vector2};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from a slice of vectors
    pub const fn from_vec_slice(s: &[Vector2<T>]) -> Rect<T>
    where
        T: [const] Clone,
    {
        assert!(s.len() >= 2);
        Rect::new(s[0].clone(), s[1].clone())
    }
}

impl<T: [const] Clone> const From<&[Vector2<T>]> for Rect<T> {
    fn from(slice: &[Vector2<T>]) -> Self {
        Rect::from_vec_slice(slice)
    }
}

impl<T: [const] Clone> const From<&[Vector2<T>; 2]> for Rect<T> {
    fn from(slice: &[Vector2<T>; 2]) -> Self {
        Rect::from_vec_slice(slice)
    }
}
