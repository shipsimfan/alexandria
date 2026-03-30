use crate::gpu::VulkanDeviceFeatures;
use vulkan::VkPhysicalDeviceFeatures2;

impl VulkanDeviceFeatures {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::gpu::device::builder::extended_info) fn to_vk(
        &self,
    ) -> VkPhysicalDeviceFeatures2 {
        VkPhysicalDeviceFeatures2::default()
    }
}
