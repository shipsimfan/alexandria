use crate::{GraphicsExtension, GraphicsVersion};

impl GraphicsExtension {
    /// Get the name of this extension
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the version of this extension
    pub fn version(&self) -> GraphicsVersion {
        self.version
    }
}
