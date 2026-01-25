use crate::{GraphicsDeviceExtendedCreateInfo, device::VkGraphicsDeviceExtendedCreateInfo};

impl GraphicsDeviceExtendedCreateInfo {
    /// Convert this exteneded creation info into its Vulkan counter-part
    pub(in crate::device) fn to_vk(&self) -> VkGraphicsDeviceExtendedCreateInfo {
        match self {
            GraphicsDeviceExtendedCreateInfo::Features(features) => features.into(),
            GraphicsDeviceExtendedCreateInfo::Vulkan13Features(features) => features.into(),
            GraphicsDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                features.into()
            }
        }
    }
}
