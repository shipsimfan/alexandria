use crate::gpu::device::VkVulkanDeviceExtendedCreateInfo;

impl VkVulkanDeviceExtendedCreateInfo {
    pub fn set_next(&mut self, next: &mut VkVulkanDeviceExtendedCreateInfo) {
        match self {
            VkVulkanDeviceExtendedCreateInfo::Features(features) => {
                features.next = next.as_mut_ptr();
            }
            VkVulkanDeviceExtendedCreateInfo::Vulkan13Features(features) => {
                features.next = next.as_mut_ptr();
            }
            VkVulkanDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features) => {
                features.next = next.as_mut_ptr();
            }
        }
    }
}
