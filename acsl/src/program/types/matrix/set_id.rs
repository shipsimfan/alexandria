use crate::program::types::Matrix;

impl Matrix {
    /// Set the id of this matrix type
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
