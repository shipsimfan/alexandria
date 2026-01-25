use crate::GraphicsDeviceFeatures;
use vulkan::VkPhysicalDeviceFeatures2;

impl GraphicsDeviceFeatures {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::device::builder::extended_info) fn to_vk(&self) -> VkPhysicalDeviceFeatures2 {
        VkPhysicalDeviceFeatures2::default()
    }
}
