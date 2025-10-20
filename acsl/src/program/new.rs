use crate::{program::types::TypeManager, Program};

impl Program {
    /// Create a new, emtpy [`Program`]
    pub(crate) fn new() -> Self {
        Program {
            types: TypeManager::new(),
        }
    }
}
