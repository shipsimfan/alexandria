use crate::gpu::VulkanAdapterMemoryProperties;
use vulkan::VkPhysicalDeviceMemoryProperties;

impl VulkanAdapterMemoryProperties {
    /// Create a new [`VulkanAdapterMemoryProperties`]
    pub(in crate::gpu::instance::adapter) fn new(
        inner: VkPhysicalDeviceMemoryProperties,
    ) -> VulkanAdapterMemoryProperties {
        VulkanAdapterMemoryProperties { inner }
    }
}
