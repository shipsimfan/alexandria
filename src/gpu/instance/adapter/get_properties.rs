use crate::gpu::{VulkanAdapter, VulkanAdapterProperties};
use vulkan::VkPhysicalDeviceProperties;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the properties of the adapter
    pub fn get_properties(&self) -> VulkanAdapterProperties {
        let mut properties = VkPhysicalDeviceProperties::default();

        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_properties)(self.handle, &mut properties)
        };

        VulkanAdapterProperties::new(properties)
    }
}
