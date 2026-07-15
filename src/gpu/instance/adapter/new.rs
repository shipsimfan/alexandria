use crate::gpu::{VulkanAdapter, VulkanInstance};
use vulkan::VkPhysicalDevice;

impl<'instance> VulkanAdapter<'instance> {
    /// Create a new [`VulkanAdapter`]
    pub(in crate::gpu::instance) fn new(
        instance: &'instance VulkanInstance,
        handle: VkPhysicalDevice,
    ) -> VulkanAdapter<'instance> {
        VulkanAdapter { handle, instance }
    }
}
