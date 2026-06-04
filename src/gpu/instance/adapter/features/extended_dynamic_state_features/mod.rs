use crate::gpu::VulkanAdapterFeature;
use vulkan::ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt;

mod get;
mod next_chain;
mod set;

/// Extended dynamic state features
#[derive(Default, Clone)]
pub struct VulkanDeviceExtendedDynamicStateFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceExtendedDynamicStateFeaturesExt,
}

impl VulkanAdapterFeature for VulkanDeviceExtendedDynamicStateFeatures {}
