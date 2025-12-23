use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from a slice of row slices
    pub const fn from_slice_rows(rows: &[&[T]]) -> Self
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(rows.len() >= 4);
        assert!(rows[0].len() >= 4);
        assert!(rows[1].len() >= 4);
        assert!(rows[2].len() >= 4);
        assert!(rows[3].len() >= 4);

        Matrix4x4::from_array_rows([
            [
                rows[0][0].clone(),
                rows[0][1].clone(),
                rows[0][2].clone(),
                rows[0][3].clone(),
            ],
            [
                rows[1][0].clone(),
                rows[1][1].clone(),
                rows[1][2].clone(),
                rows[1][3].clone(),
            ],
            [
                rows[2][0].clone(),
                rows[2][1].clone(),
                rows[2][2].clone(),
                rows[2][3].clone(),
            ],
            [
                rows[3][0].clone(),
                rows[3][1].clone(),
                rows[3][2].clone(),
                rows[3][3].clone(),
            ],
        ])
    }

    /// Create a [`Matrix4x4`] from a slice of column slices
    pub const fn from_slice_cols(cols: &[&[T]]) -> Self
    where
        T: [const] Clone + [const] Destruct,
    {
        assert!(cols.len() >= 4);
        assert!(cols[0].len() >= 4);
        assert!(cols[1].len() >= 4);
        assert!(cols[2].len() >= 4);
        assert!(cols[3].len() >= 4);

        Matrix4x4::from_array_cols([
            [
                cols[0][0].clone(),
                cols[0][1].clone(),
                cols[0][2].clone(),
                cols[0][3].clone(),
            ],
            [
                cols[1][0].clone(),
                cols[1][1].clone(),
                cols[1][2].clone(),
                cols[1][3].clone(),
            ],
            [
                cols[2][0].clone(),
                cols[2][1].clone(),
                cols[2][2].clone(),
                cols[2][3].clone(),
            ],
            [
                cols[3][0].clone(),
                cols[3][1].clone(),
                cols[3][2].clone(),
                cols[3][3].clone(),
            ],
        ])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[&[T]]> for Matrix4x4<T> {
    fn from(rows: &[&[T]]) -> Self {
        Matrix4x4::from_slice_rows(rows)
    }
}
