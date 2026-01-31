use crate::math::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Get the value at `row` and `col`
    pub const fn get(&self, row: usize, col: usize) -> Option<T>
    where
        T: [const] Clone,
    {
        self.get_ref(row, col).map(Clone::clone)
    }
}
