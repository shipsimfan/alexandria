use crate::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Get a reference to `col`
    pub const fn col_ref(&self, col: usize) -> Option<[&T; 3]> {
        match col {
            0 => Some([&self.r0.x, &self.r1.x, &self.r2.x]),
            1 => Some([&self.r0.y, &self.r1.y, &self.r2.y]),
            2 => Some([&self.r0.z, &self.r1.z, &self.r2.z]),
            _ => None,
        }
    }

    /// Get a mutable reference to `col`
    pub const fn col_ref_mut(&mut self, col: usize) -> Option<[&mut T; 3]> {
        match col {
            0 => Some([&mut self.r0.x, &mut self.r1.x, &mut self.r2.x]),
            1 => Some([&mut self.r0.y, &mut self.r1.y, &mut self.r2.y]),
            2 => Some([&mut self.r0.z, &mut self.r1.z, &mut self.r2.z]),
            _ => None,
        }
    }
}
