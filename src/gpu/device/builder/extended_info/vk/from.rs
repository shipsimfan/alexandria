use crate::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
    device::VkVulkanDeviceExtendedCreateInfo,
};

impl From<&VulkanDeviceFeatures> for VkVulkanDeviceExtendedCreateInfo {
    fn from(features: &VulkanDeviceFeatures) -> Self {
        VkVulkanDeviceExtendedCreateInfo::Features(features.to_vk())
    }
}

impl From<&VulkanDeviceVulkan13Features> for VkVulkanDeviceExtendedCreateInfo {
    fn from(features: &VulkanDeviceVulkan13Features) -> Self {
        VkVulkanDeviceExtendedCreateInfo::Vulkan13Features(features.to_vk())
    }
}

impl From<&VulkanDeviceExtendedDynamicStateFeatures> for VkVulkanDeviceExtendedCreateInfo {
    fn from(features: &VulkanDeviceExtendedDynamicStateFeatures) -> Self {
        VkVulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features.to_vk())
    }
}
