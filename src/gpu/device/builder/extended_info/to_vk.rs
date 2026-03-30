use crate::gpu::{VulkanDeviceExtendedCreateInfo, device::VkVulkanDeviceExtendedCreateInfo};

impl VulkanDeviceExtendedCreateInfo {
    /// Convert this exteneded creation info into its Vulkan counter-part
    pub(in crate::gpu::device) fn to_vk(&self) -> VkVulkanDeviceExtendedCreateInfo {
        match self {
            VulkanDeviceExtendedCreateInfo::Features(features) => features.into(),
            VulkanDeviceExtendedCreateInfo::Vulkan13Features(features) => features.into(),
            VulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                features.into()
            }
        }
    }
}
