use crate::device::VkGraphicsDeviceExtendedCreateInfo;

impl VkGraphicsDeviceExtendedCreateInfo {
    pub fn set_next(&mut self, next: *const VkGraphicsDeviceExtendedCreateInfo) {
        match self {
            VkGraphicsDeviceExtendedCreateInfo::Features(features) => {
                features.next = next.cast_mut().cast();
            }
            VkGraphicsDeviceExtendedCreateInfo::Vulkan13Features(features) => {
                features.next = next.cast_mut().cast();
            }
            VkGraphicsDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                features.next = next.cast_mut().cast();
            }
        }
    }
}
