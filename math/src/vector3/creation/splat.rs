use crate::Vector3;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`] with all values being `v`
    pub const fn splat(v: T) -> Self
    where
        T: [const] Clone,
    {
        Vector3::new(v.clone(), v.clone(), v)
    }
}
