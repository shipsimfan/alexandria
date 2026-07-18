use crate::math::Matrix4x4;

const impl<T: [const] Clone> Clone for Matrix4x4<T> {
    fn clone(&self) -> Self {
        Matrix4x4::new_rows(
            self.r0.clone(),
            self.r1.clone(),
            self.r2.clone(),
            self.r3.clone(),
        )
    }
}

impl<T: Copy> Copy for Matrix4x4<T> {}
