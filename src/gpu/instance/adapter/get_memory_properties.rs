use crate::gpu::{VulkanAdapter, VulkanAdapterMemoryProperties};
use vulkan::VkPhysicalDeviceMemoryProperties;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the memory properties of the adapter
    pub fn get_memory_properties(&self) -> VulkanAdapterMemoryProperties {
        let mut memory_properties = VkPhysicalDeviceMemoryProperties::default();
        unsafe {
            (self
                .instance
                .functions()
                .adapter
                .get_physical_device_memory_properties)(
                self.handle, &mut memory_properties
            )
        };
        VulkanAdapterMemoryProperties::new(memory_properties)
    }
}
