use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Map all the elements of this matrix component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Matrix4x4<U>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            self.r0.map(&mut f),
            self.r1.map(&mut f),
            self.r2.map(&mut f),
            self.r3.map(f),
        )
    }
}
