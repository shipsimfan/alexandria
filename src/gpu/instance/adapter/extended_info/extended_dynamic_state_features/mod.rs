use vulkan::ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt;

mod as_ptr;
mod get;
mod set;
mod set_next;

/// Extended dynamic state features
#[derive(Default, Clone)]
pub struct VulkanDeviceExtendedDynamicStateFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceExtendedDynamicStateFeaturesExt,
}
