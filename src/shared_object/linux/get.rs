use crate::shared_object::SharedObjectInner;
use std::path::Path;

impl SharedObjectInner {
    /// Get the name of this shared object
    pub fn name(&self) -> &Path {
        &self.name
    }
}
