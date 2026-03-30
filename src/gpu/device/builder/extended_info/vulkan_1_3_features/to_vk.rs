use crate::gpu::VulkanDeviceVulkan13Features;
use vulkan::VkPhysicalDeviceVulkan13Features;

impl VulkanDeviceVulkan13Features {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::gpu::device::builder::extended_info) fn to_vk(
        &self,
    ) -> VkPhysicalDeviceVulkan13Features {
        VkPhysicalDeviceVulkan13Features::default()
    }
}
