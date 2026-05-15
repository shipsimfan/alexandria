use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;

impl Default for VulkanDeviceExtendedDynamicStateFeatures {
    fn default() -> Self {
        VulkanDeviceExtendedDynamicStateFeatures {
            extended_dynamic_state: false,
        }
    }
}
