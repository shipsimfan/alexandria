use crate::math::{Matrix3x3, Vector3};

impl<T: Clone> Matrix3x3<T> {
    /// Get the column at `col`
    pub fn col(&self, col: usize) -> Vector3<T> {
        match self.try_col(col) {
            Some(col) => col,
            None => panic!("index out of bounds: the len is 3 but the index is {}", col),
        }
    }
}

impl<T> Matrix3x3<T> {
    /// Get the column at `col`
    pub const fn try_col(&self, col: usize) -> Option<Vector3<T>>
    where
        T: [const] Clone,
    {
        match col {
            0 => Some(Vector3::new(
                self.r0.x.clone(),
                self.r1.x.clone(),
                self.r2.x.clone(),
            )),
            1 => Some(Vector3::new(
                self.r0.y.clone(),
                self.r1.y.clone(),
                self.r2.y.clone(),
            )),
            2 => Some(Vector3::new(
                self.r0.z.clone(),
                self.r1.z.clone(),
                self.r2.z.clone(),
            )),
            _ => None,
        }
    }
}
