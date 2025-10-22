use crate::{program::types::TypeManager, Program};

impl Program {
    /// Get the types that make up this program
    pub fn types(&self) -> &TypeManager {
        &self.types
    }
}
