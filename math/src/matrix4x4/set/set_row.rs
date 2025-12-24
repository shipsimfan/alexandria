use crate::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Set the row at `row` to `value`
    pub fn set_row(&mut self, row: usize, value: Vector4<T>) {
        match self.row_ref_mut(row) {
            Some(row) => *row = value,
            None => panic!("index out of bounds: the len is 4 but the index is {}", row),
        }
    }
}
