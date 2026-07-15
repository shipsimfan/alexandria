use crate::gpu::VulkanAdapterProperties;
use vulkan::VkPhysicalDeviceProperties;

impl VulkanAdapterProperties {
    /// Create a new [`VulkanAdapterProperties`]
    pub(in crate::gpu::instance::adapter) fn new(
        inner: VkPhysicalDeviceProperties,
    ) -> VulkanAdapterProperties {
        VulkanAdapterProperties { inner }
    }
}
