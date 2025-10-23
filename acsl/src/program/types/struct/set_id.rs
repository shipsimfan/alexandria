use crate::program::types::Struct;

impl Struct {
    /// Set the id of this struct type
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
