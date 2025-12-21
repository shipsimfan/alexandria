use crate::Vector4;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`] with all values being `v`
    pub const fn splat(v: T) -> Self
    where
        T: [const] Clone,
    {
        Vector4::new(v.clone(), v.clone(), v.clone(), v)
    }
}
