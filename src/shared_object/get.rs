use crate::SharedObject;
use std::path::Path;

impl SharedObject {
    /// Get the name of this shared object
    pub fn name(&self) -> &Path {
        self.inner.name()
    }
}
