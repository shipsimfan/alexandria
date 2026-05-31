use crate::math::{ColorHsv, ColorSpace};

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Get this color as a slice in the order `[h, s, v]`
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.h, 3) }
    }

    /// Get this color as a mutable slice in the order `[h, s, v]`
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.h, 3) }
    }
}
