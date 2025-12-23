use crate::Vector4;

impl<T> Vector4<T> {
    /// Get a reference to the value at `i`
    pub const fn get_ref(&self, i: usize) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            3 => Some(&self.w),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `i`
    pub const fn get_ref_mut(&mut self, i: usize) -> Option<&mut T> {
        match i {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            3 => Some(&mut self.w),
            _ => None,
        }
    }
}
