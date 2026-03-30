use crate::gpu::{VulkanAdapter, VulkanDeviceBuilder};

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`VulkanDeviceBuilder`]
    pub(in crate::gpu) fn new(
        adapter: &'adapter VulkanAdapter<'instance>,
    ) -> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        VulkanDeviceBuilder {
            extended_info: Vec::new(),
            queues: Vec::new(),
            extensions: Vec::new(),
            adapter,
        }
    }
}
