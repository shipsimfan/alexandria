use crate::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Get a reference to `row`
    pub const fn row_ref(&self, row: usize) -> Option<&Vector4<T>> {
        match row {
            0 => Some(&self.r0),
            1 => Some(&self.r1),
            2 => Some(&self.r2),
            3 => Some(&self.r3),
            _ => None,
        }
    }

    /// Get a mutable reference to `row`
    pub const fn row_mut_ref(&mut self, row: usize) -> Option<&mut Vector4<T>> {
        match row {
            0 => Some(&mut self.r0),
            1 => Some(&mut self.r1),
            2 => Some(&mut self.r2),
            3 => Some(&mut self.r3),
            _ => None,
        }
    }
}
