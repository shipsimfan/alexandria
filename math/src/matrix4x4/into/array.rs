use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Convert the elements of this [`Matrix4x4`] into a row-major array
    pub const fn into_row_array(self) -> [[T; 4]; 4]
    where
        T: [const] Destruct,
    {
        [
            self.r0.into(),
            self.r1.into(),
            self.r2.into(),
            self.r3.into(),
        ]
    }

    /// Convert the elements of this [`Matrix4x4`] into a column-major array
    pub const fn into_col_array(self) -> [[T; 4]; 4]
    where
        T: [const] Destruct,
    {
        [
            [self.r0.x, self.r1.x, self.r2.x, self.r3.x],
            [self.r0.y, self.r1.y, self.r2.y, self.r3.y],
            [self.r0.z, self.r1.z, self.r2.z, self.r3.z],
            [self.r0.w, self.r1.w, self.r2.w, self.r3.w],
        ]
    }
}

impl<T: [const] Destruct> const Into<[[T; 4]; 4]> for Matrix4x4<T> {
    fn into(self) -> [[T; 4]; 4] {
        self.into_row_array()
    }
}
