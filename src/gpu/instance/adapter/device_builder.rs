use crate::gpu::{VulkanAdapter, VulkanDeviceBuilder};

impl<'instance> VulkanAdapter<'instance> {
    /// Create a new [`VulkanDeviceBuilder`]
    pub fn device_builder<'adapter, 'a>(
        &'adapter self,
    ) -> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
        VulkanDeviceBuilder::new(self)
    }
}
