use crate::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Set the row at `row` to `value`
    pub fn set_row(&mut self, row: usize, value: Vector3<T>) {
        match self.row_ref_mut(row) {
            Some(row) => *row = value,
            None => panic!("index out of bounds: the len is 3 but the index is {}", row),
        }
    }
}
