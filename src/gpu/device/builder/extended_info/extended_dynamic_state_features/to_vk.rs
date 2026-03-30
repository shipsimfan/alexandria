use crate::gpu::VulkanDeviceExtendedDynamicStateFeatures;
use vulkan::ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt;

impl VulkanDeviceExtendedDynamicStateFeatures {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::gpu::device::builder::extended_info) fn to_vk(
        &self,
    ) -> VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
        VkPhysicalDeviceExtendedDynamicStateFeaturesExt::default()
    }
}
