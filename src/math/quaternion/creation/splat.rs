use crate::math::Quaternion;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`] containing only `v`
    pub const fn splat(v: T) -> Quaternion<T>
    where
        T: [const] Clone,
    {
        Quaternion::new(v.clone(), v.clone(), v.clone(), v)
    }
}
