use crate::math::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Get a reference to the value at `row` and `col`
    pub const fn get_ref(&self, row: usize, col: usize) -> Option<&T> {
        match row {
            0 => self.r0.get_ref(col),
            1 => self.r1.get_ref(col),
            2 => self.r2.get_ref(col),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `row` and `col`
    pub const fn get_ref_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        match row {
            0 => self.r0.get_ref_mut(col),
            1 => self.r1.get_ref_mut(col),
            2 => self.r2.get_ref_mut(col),
            _ => None,
        }
    }
}
