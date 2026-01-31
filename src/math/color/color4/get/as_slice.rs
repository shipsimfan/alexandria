use crate::math::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Get this color as a slice in the order `[r, g, b, a]`
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.r, 4) }
    }

    /// Get this color as a mutable slice in the order `[r, g, b, a]`
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.r, 4) }
    }
}
