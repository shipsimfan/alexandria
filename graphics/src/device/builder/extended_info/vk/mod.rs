use vulkan::{
    VkPhysicalDeviceFeatures2, VkPhysicalDeviceVulkan13Features,
    ext_extended_dynamic_state::VkPhysicalDeviceExtendedDynamicStateFeaturesExt,
};

mod from;
mod set_next;

/// Extended information that can be provided when creating a graphics device
pub(in crate::device) enum VkGraphicsDeviceExtendedCreateInfo {
    /// The general device features
    Features(VkPhysicalDeviceFeatures2),

    /// Vulkan 1.3 specific features
    Vulkan13Features(VkPhysicalDeviceVulkan13Features),

    /// Dynamic rendering features
    ExtendedDynamicStateFeatures(VkPhysicalDeviceExtendedDynamicStateFeaturesExt),
}
