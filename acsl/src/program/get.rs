use crate::{program::types::TypeManager, Program};

impl Program {
    /// Get the types that make up this program
    pub const fn types(&self) -> &TypeManager {
        &self.types
    }

    /// Get a mutable reference to the type manager
    pub(crate) fn types_mut(&mut self) -> &mut TypeManager {
        &mut self.types
    }
}
