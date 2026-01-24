use crate::GraphicsDeviceVulkan13Features;
use vulkan::VkPhysicalDeviceVulkan13Features;

impl GraphicsDeviceVulkan13Features {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::device::builder::extended_info) fn into_vk(
        &self,
    ) -> VkPhysicalDeviceVulkan13Features {
        VkPhysicalDeviceVulkan13Features::default()
    }
}
