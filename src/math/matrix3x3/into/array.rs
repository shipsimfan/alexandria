use crate::math::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Convert the elements of this [`Matrix3x3`] into a row-major array
    pub const fn into_row_array(self) -> [[T; 3]; 3]
    where
        T: [const] Destruct,
    {
        [self.r0.into(), self.r1.into(), self.r2.into()]
    }

    /// Convert the elements of this [`Matrix3x3`] into a column-major array
    pub const fn into_col_array(self) -> [[T; 3]; 3]
    where
        T: [const] Destruct,
    {
        [
            [self.r0.x, self.r1.x, self.r2.x],
            [self.r0.y, self.r1.y, self.r2.y],
            [self.r0.z, self.r1.z, self.r2.z],
        ]
    }
}

impl<T: [const] Destruct> const Into<[[T; 3]; 3]> for Matrix3x3<T> {
    fn into(self) -> [[T; 3]; 3] {
        self.into_row_array()
    }
}
