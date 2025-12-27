use crate::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Get a reference to `row`
    pub const fn row_ref(&self, row: usize) -> Option<&Vector3<T>> {
        match row {
            0 => Some(&self.r0),
            1 => Some(&self.r1),
            2 => Some(&self.r2),
            _ => None,
        }
    }

    /// Get a mutable reference to `row`
    pub const fn row_ref_mut(&mut self, row: usize) -> Option<&mut Vector3<T>> {
        match row {
            0 => Some(&mut self.r0),
            1 => Some(&mut self.r1),
            2 => Some(&mut self.r2),
            _ => None,
        }
    }
}
