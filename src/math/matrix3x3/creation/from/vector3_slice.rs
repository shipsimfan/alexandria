use crate::math::{Matrix3x3, Vector3};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from borrowed [`Vector3`] rows
    pub const fn from_vec3_row_slice(rows: &[Vector3<T>]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(rows.len() >= 3);
        Matrix3x3::from_rows([rows[0].clone(), rows[1].clone(), rows[2].clone()])
    }

    /// Create a [`Matrix3x3`] from borrowed [`Vector3`] columns
    pub const fn from_vec3_col_slice(cols: &[Vector3<T>]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(cols.len() >= 3);
        Matrix3x3::from_cols([cols[0].clone(), cols[1].clone(), cols[2].clone()])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[Vector3<T>]> for Matrix3x3<T> {
    fn from(rows: &[Vector3<T>]) -> Self {
        Matrix3x3::from_vec3_row_slice(rows)
    }
}
