use vulkan::{
    VkPhysicalDeviceFeatures2, VkPhysicalDeviceVulkan13Features,
    ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt,
};

mod as_ptr;
mod from;
mod set_next;

/// Extended information that can be provided when creating a graphics device
pub(in crate::gpu::device) enum VkVulkanDeviceExtendedCreateInfo {
    /// The general device features
    Features(VkPhysicalDeviceFeatures2),

    /// Vulkan 1.3 specific features
    Vulkan13Features(VkPhysicalDeviceVulkan13Features),

    /// Dynamic rendering features
    ExtendedDynamicStateFeatures(VkPhysicalDeviceExtendedDynamicStateFeaturesExt),
}
