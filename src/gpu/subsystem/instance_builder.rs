use crate::gpu::{GpuSubsystem, VulkanInstanceBuilder, VulkanVersion};

impl GpuSubsystem {
    /// Create a new [`VulkanInstanceBuilder`]
    pub fn instance_builder<'a>(&self, api_version: VulkanVersion) -> VulkanInstanceBuilder<'a> {
        VulkanInstanceBuilder::new(self.clone(), api_version)
    }
}
