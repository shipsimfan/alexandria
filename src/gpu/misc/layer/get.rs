use crate::gpu::{VulkanLayer, VulkanVersion};

impl VulkanLayer {
    /// Get the name of this layer
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a description of the layer
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the specification version of the layer implemented
    pub fn spec_version(&self) -> VulkanVersion {
        self.spec_version
    }

    /// Get the driver reported version of the layer
    pub fn version(&self) -> VulkanVersion {
        self.version
    }
}
