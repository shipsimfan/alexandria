use crate::Vector2;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] with all values being `v`
    pub const fn splat(v: T) -> Vector2<T>
    where
        T: [const] Clone,
    {
        Vector2::new(v.clone(), v)
    }
}
