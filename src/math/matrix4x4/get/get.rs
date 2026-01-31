use crate::math::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Get the value at `row` and `col`
    pub const fn get(&self, row: usize, col: usize) -> Option<T>
    where
        T: [const] Clone,
    {
        self.get_ref(row, col).map(Clone::clone)
    }
}
