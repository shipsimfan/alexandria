use crate::math::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from borrowed [`Vector4`] rows
    pub const fn from_vec4_row_slice(rows: &[Vector4<T>]) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(rows.len() >= 4);
        Matrix4x4::from_rows([
            rows[0].clone(),
            rows[1].clone(),
            rows[2].clone(),
            rows[3].clone(),
        ])
    }

    /// Create a [`Matrix4x4`] from borrowed [`Vector4`] columns
    pub const fn from_vec4_col_slice(cols: &[Vector4<T>]) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(cols.len() >= 4);
        Matrix4x4::from_cols([
            cols[0].clone(),
            cols[1].clone(),
            cols[2].clone(),
            cols[3].clone(),
        ])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[Vector4<T>]> for Matrix4x4<T> {
    fn from(rows: &[Vector4<T>]) -> Self {
        Matrix4x4::from_vec4_row_slice(rows)
    }
}
