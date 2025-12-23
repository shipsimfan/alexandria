use crate::{Matrix4x4, Vector4};

impl<T: Clone> Matrix4x4<T> {
    /// Get the row at `row`
    pub fn row(&self, row: usize) -> Vector4<T> {
        match self.try_row(row) {
            Some(row) => row,
            None => panic!("index out of bounds: the len is 4 but the index is {}", row),
        }
    }
}

impl<T> Matrix4x4<T> {
    /// Get the row at `row`
    pub const fn try_row(&self, row: usize) -> Option<Vector4<T>>
    where
        T: [const] Clone,
    {
        self.row_ref(row).map(Clone::clone)
    }
}
