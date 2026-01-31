use crate::math::{Matrix4x4, Vector4};

impl<T: Clone> Matrix4x4<T> {
    /// Get the column at `col`
    pub fn col(&self, col: usize) -> Vector4<T> {
        match self.try_col(col) {
            Some(col) => col,
            None => panic!("index out of bounds: the len is 4 but the index is {}", col),
        }
    }
}

impl<T> Matrix4x4<T> {
    /// Get the column at `col`
    pub const fn try_col(&self, col: usize) -> Option<Vector4<T>>
    where
        T: [const] Clone,
    {
        match col {
            0 => Some(Vector4::new(
                self.r0.x.clone(),
                self.r1.x.clone(),
                self.r2.x.clone(),
                self.r3.x.clone(),
            )),
            1 => Some(Vector4::new(
                self.r0.y.clone(),
                self.r1.y.clone(),
                self.r2.y.clone(),
                self.r3.y.clone(),
            )),
            2 => Some(Vector4::new(
                self.r0.z.clone(),
                self.r1.z.clone(),
                self.r2.z.clone(),
                self.r3.z.clone(),
            )),
            3 => Some(Vector4::new(
                self.r0.w.clone(),
                self.r1.w.clone(),
                self.r2.w.clone(),
                self.r3.w.clone(),
            )),
            _ => None,
        }
    }
}
