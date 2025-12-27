use crate::Matrix3x3;

impl<T: [const] Clone> const Clone for Matrix3x3<T> {
    fn clone(&self) -> Self {
        Matrix3x3::new_rows(self.r0.clone(), self.r1.clone(), self.r2.clone())
    }
}

impl<T: Copy> Copy for Matrix3x3<T> {}
