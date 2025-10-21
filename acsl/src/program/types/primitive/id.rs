use crate::program::types::Primitive;

impl Primitive {
    /// Get the id of this [`Primitive`] type
    pub const fn id(&self) -> u32 {
        match self {
            Primitive::F32 => 0,
            Primitive::U32 => 1,
        }
    }

    /// Set the id of this [`Primitive`] type
    pub(in crate::program::types) unsafe fn set_id(&mut self, id: u32) {
        assert_eq!(id, self.id());
    }
}
