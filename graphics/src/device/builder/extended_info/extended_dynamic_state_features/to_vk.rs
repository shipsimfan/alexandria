use crate::GraphicsDeviceExtendedDynamicStateFeatures;
use vulkan::ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt;

impl GraphicsDeviceExtendedDynamicStateFeatures {
    /// Convert this structure into its Vulkan counterpart
    pub(in crate::device::builder::extended_info) fn to_vk(
        &self,
    ) -> VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
        VkPhysicalDeviceExtendedDynamicStateFeaturesExt::default()
    }
}
