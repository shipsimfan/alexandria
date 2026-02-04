use crate::{
    AlexandriaContext,
    gpu::{GpuSubsystem, VulkanInstanceBuilder, VulkanVersion},
};

impl GpuSubsystem {
    /// Create a new [`VulkanInstanceBuilder`]
    pub fn instance_builder<'a>(&self, api_version: VulkanVersion) -> VulkanInstanceBuilder<'a> {
        VulkanInstanceBuilder::new(AlexandriaContext::from_weak(&self.context), api_version)
    }
}
