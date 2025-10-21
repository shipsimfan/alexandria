use crate::program::types::Vector;

impl Vector {
    /// Set the id of this vector type
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
