use crate::gpu::device::VkVulkanDeviceExtendedCreateInfo;

impl VkVulkanDeviceExtendedCreateInfo {
    pub fn set_next(&mut self, next: *const VkVulkanDeviceExtendedCreateInfo) {
        match self {
            VkVulkanDeviceExtendedCreateInfo::Features(features) => {
                features.next = next.cast_mut().cast();
            }
            VkVulkanDeviceExtendedCreateInfo::Vulkan13Features(features) => {
                features.next = next.cast_mut().cast();
            }
            VkVulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                features.next = next.cast_mut().cast();
            }
        }
    }
}
