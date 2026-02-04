use crate::{FunctionSymbol, SharedObject};

impl<F> FunctionSymbol<F> {
    /// Get a reference to the shared object that this symbol came from
    pub const fn shared_object(&self) -> &SharedObject {
        &self.shared_object
    }
}
