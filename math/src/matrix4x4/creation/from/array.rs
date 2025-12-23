use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from an array of row arrays
    pub const fn from_array_rows([r0, r1, r2, r3]: [[T; 4]; 4]) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::from_rows([r0.into(), r1.into(), r2.into(), r3.into()])
    }

    /// Create a [`Matrix4x4`] from an array of column arrays
    pub const fn from_array_cols([c0, c1, c2, c3]: [[T; 4]; 4]) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::from_cols([c0.into(), c1.into(), c2.into(), c3.into()])
    }
}

impl<T: [const] Destruct> const From<[[T; 4]; 4]> for Matrix4x4<T> {
    fn from(rows: [[T; 4]; 4]) -> Self {
        Matrix4x4::from_array_rows(rows)
    }
}
