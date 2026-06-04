use crate::gpu::{GpuSubsystem, VulkanInstanceBuilder, VulkanInstanceCreateFlags, VulkanVersion};

impl<'a> VulkanInstanceBuilder<'a> {
    /// Create a new [`VulkanInstanceBuilder`]
    pub(in crate::gpu) fn new(
        context: GpuSubsystem,
        api_version: VulkanVersion,
    ) -> VulkanInstanceBuilder<'a> {
        VulkanInstanceBuilder {
            context,
            flags: VulkanInstanceCreateFlags::empty(),
            api_version,
            application: None,
            engine: None,
            extensions: Vec::new(),
            layers: Vec::new(),
        }
    }
}
