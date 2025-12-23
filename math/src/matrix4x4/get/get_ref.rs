use crate::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Get the value at `row` and `col`
    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        todo!()
    }

    /// Get the value at `row` and `col`
    pub const fn try_get_ref(&self, row: usize, col: usize) -> Option<&T> {
        todo!()
    }

    /// Get the value at `row` and `col`
    pub fn get_mut_ref(&mut self, row: usize, col: usize) -> &mut T {
        todo!()
    }

    /// Get the value at `row` and `col`
    pub const fn try_get_mut_ref(&mut self, row: usize, col: usize) -> Option<&mut T> {
        todo!()
    }
}
