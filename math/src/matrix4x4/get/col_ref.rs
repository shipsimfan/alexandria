use crate::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Get a reference to `col`
    pub const fn col_ref(&self, col: usize) -> Option<[&T; 4]> {
        match col {
            0 => Some([&self.r0.x, &self.r1.x, &self.r2.x, &self.r3.x]),
            1 => Some([&self.r0.y, &self.r1.y, &self.r2.y, &self.r3.y]),
            2 => Some([&self.r0.z, &self.r1.z, &self.r2.z, &self.r3.z]),
            3 => Some([&self.r0.w, &self.r1.w, &self.r2.w, &self.r3.w]),
            _ => None,
        }
    }

    /// Get a mutable reference to `col`
    pub const fn col_ref_mut(&mut self, col: usize) -> Option<[&mut T; 4]> {
        match col {
            0 => Some([
                &mut self.r0.x,
                &mut self.r1.x,
                &mut self.r2.x,
                &mut self.r3.x,
            ]),
            1 => Some([
                &mut self.r0.y,
                &mut self.r1.y,
                &mut self.r2.y,
                &mut self.r3.y,
            ]),
            2 => Some([
                &mut self.r0.z,
                &mut self.r1.z,
                &mut self.r2.z,
                &mut self.r3.z,
            ]),
            3 => Some([
                &mut self.r0.w,
                &mut self.r1.w,
                &mut self.r2.w,
                &mut self.r3.w,
            ]),
            _ => None,
        }
    }
}
