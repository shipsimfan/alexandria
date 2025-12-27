use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Map all the elements of this matrix component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Matrix3x3<U>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            self.r0.map(&mut f),
            self.r1.map(&mut f),
            self.r2.map(&mut f),
        )
    }
}
