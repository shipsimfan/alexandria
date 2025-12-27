use crate::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Creates a new [`Matrix3x3`] containing all elements set to `v`
    pub const fn splat(v: T) -> Matrix3x3<T>
    where
        T: [const] Clone,
    {
        Matrix3x3::new_rows(
            Vector3::splat(v.clone()),
            Vector3::splat(v.clone()),
            Vector3::splat(v),
        )
    }
}
