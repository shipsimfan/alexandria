use crate::gpu::{
    VulkanDeviceExtendedCreateInfo, VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures,
    VulkanDeviceVulkan13Features,
};

impl From<VulkanDeviceFeatures> for VulkanDeviceExtendedCreateInfo {
    fn from(features: VulkanDeviceFeatures) -> Self {
        VulkanDeviceExtendedCreateInfo::Features(features)
    }
}

impl From<VulkanDeviceVulkan13Features> for VulkanDeviceExtendedCreateInfo {
    fn from(features: VulkanDeviceVulkan13Features) -> Self {
        VulkanDeviceExtendedCreateInfo::Vulkan13Features(features)
    }
}

impl From<VulkanDeviceExtendedDynamicStateFeatures> for VulkanDeviceExtendedCreateInfo {
    fn from(features: VulkanDeviceExtendedDynamicStateFeatures) -> Self {
        VulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features)
    }
}
