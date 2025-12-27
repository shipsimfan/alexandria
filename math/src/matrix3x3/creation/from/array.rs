use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from an array of row arrays
    pub const fn from_row_array([r0, r1, r2]: [[T; 3]; 3]) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::from_rows([r0.into(), r1.into(), r2.into()])
    }

    /// Create a [`Matrix3x3`] from an array of column arrays
    pub const fn from_col_array([c0, c1, c2]: [[T; 3]; 3]) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::from_cols([c0.into(), c1.into(), c2.into()])
    }
}

impl<T: [const] Destruct> const From<[[T; 3]; 3]> for Matrix3x3<T> {
    fn from(rows: [[T; 3]; 3]) -> Self {
        Matrix3x3::from_row_array(rows)
    }
}
