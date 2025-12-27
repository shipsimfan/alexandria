use crate::{Matrix3x3, Vector3};

impl<T: Clone> Matrix3x3<T> {
    /// Get the row at `row`
    pub fn row(&self, row: usize) -> Vector3<T> {
        match self.try_row(row) {
            Some(row) => row,
            None => panic!("index out of bounds: the len is 3 but the index is {}", row),
        }
    }
}

impl<T> Matrix3x3<T> {
    /// Get the row at `row`
    pub const fn try_row(&self, row: usize) -> Option<Vector3<T>>
    where
        T: [const] Clone,
    {
        self.row_ref(row).map(Clone::clone)
    }
}
