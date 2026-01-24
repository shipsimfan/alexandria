use crate::{
    GraphicsDeviceExtendedDynamicStateFeatures, GraphicsDeviceFeatures,
    GraphicsDeviceVulkan13Features, device::VkGraphicsDeviceExtendedCreateInfo,
};

impl From<&GraphicsDeviceFeatures> for VkGraphicsDeviceExtendedCreateInfo {
    fn from(features: &GraphicsDeviceFeatures) -> Self {
        VkGraphicsDeviceExtendedCreateInfo::Features(features.into_vk())
    }
}

impl From<&GraphicsDeviceVulkan13Features> for VkGraphicsDeviceExtendedCreateInfo {
    fn from(features: &GraphicsDeviceVulkan13Features) -> Self {
        VkGraphicsDeviceExtendedCreateInfo::Vulkan13Features(features.into_vk())
    }
}

impl From<&GraphicsDeviceExtendedDynamicStateFeatures> for VkGraphicsDeviceExtendedCreateInfo {
    fn from(features: &GraphicsDeviceExtendedDynamicStateFeatures) -> Self {
        VkGraphicsDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features.into_vk())
    }
}
