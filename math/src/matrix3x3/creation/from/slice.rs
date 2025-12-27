use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from a slice of row slices
    pub const fn from_row_slice(rows: &[&[T]]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(rows.len() >= 3);
        assert!(rows[0].len() >= 3);
        assert!(rows[1].len() >= 3);
        assert!(rows[2].len() >= 3);

        Matrix3x3::from_row_array([
            [rows[0][0].clone(), rows[0][1].clone(), rows[0][2].clone()],
            [rows[1][0].clone(), rows[1][1].clone(), rows[1][2].clone()],
            [rows[2][0].clone(), rows[2][1].clone(), rows[2][2].clone()],
        ])
    }

    /// Create a [`Matrix3x3`] from a slice of column slices
    pub const fn from_col_slice(cols: &[&[T]]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(cols.len() >= 3);
        assert!(cols[0].len() >= 3);
        assert!(cols[1].len() >= 3);
        assert!(cols[2].len() >= 3);

        Matrix3x3::from_col_array([
            [cols[0][0].clone(), cols[0][1].clone(), cols[0][2].clone()],
            [cols[1][0].clone(), cols[1][1].clone(), cols[1][2].clone()],
            [cols[2][0].clone(), cols[2][1].clone(), cols[2][2].clone()],
        ])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[&[T]]> for Matrix3x3<T> {
    fn from(rows: &[&[T]]) -> Self {
        Matrix3x3::from_row_slice(rows)
    }
}
