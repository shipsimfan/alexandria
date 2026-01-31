use crate::math::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Swap rows `a` and `b` in this matrix
    pub const fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        self.as_row_slices_mut().swap(a, b);
    }
}
