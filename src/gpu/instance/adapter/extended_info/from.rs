use crate::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
    VulkanExtendedAdapterInfo,
};

impl From<VulkanDeviceFeatures> for VulkanExtendedAdapterInfo {
    fn from(features: VulkanDeviceFeatures) -> Self {
        VulkanExtendedAdapterInfo::Features(features)
    }
}

impl From<VulkanDeviceVulkan13Features> for VulkanExtendedAdapterInfo {
    fn from(features: VulkanDeviceVulkan13Features) -> Self {
        VulkanExtendedAdapterInfo::Vulkan13Features(features)
    }
}

impl From<VulkanDeviceExtendedDynamicStateFeatures> for VulkanExtendedAdapterInfo {
    fn from(features: VulkanDeviceExtendedDynamicStateFeatures) -> Self {
        VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features)
    }
}
