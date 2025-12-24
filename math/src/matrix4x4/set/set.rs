use crate::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Sets the value at `row` and `col` to `v`
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        match self.row_ref_mut(row) {
            Some(row) => row.set(col, value),
            None => panic!("index out of bounds: the len is 4 but the index is {}", row),
        }
    }
}
