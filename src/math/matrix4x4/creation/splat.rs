use crate::math::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Creates a new [`Matrix4x4`] containing all elements set to `v`
    pub const fn splat(v: T) -> Matrix4x4<T>
    where
        T: [const] Clone,
    {
        Matrix4x4::new_rows(
            Vector4::splat(v.clone()),
            Vector4::splat(v.clone()),
            Vector4::splat(v.clone()),
            Vector4::splat(v),
        )
    }
}
