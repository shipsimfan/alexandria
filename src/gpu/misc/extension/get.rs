use crate::gpu::{VulkanExtension, VulkanVersion};

impl VulkanExtension {
    /// Get the name of this extension
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the version of this extension
    pub fn version(&self) -> VulkanVersion {
        self.version
    }
}
