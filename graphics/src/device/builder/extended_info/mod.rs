mod vk;

mod extended_dynamic_state_features;
mod into_vk;
mod vulkan_1_3_features;

pub(in crate::device) use vk::VkGraphicsDeviceExtendedCreateInfo;

/// Extended information that can be provided when creating a graphics device
pub enum GraphicsDeviceExtendedCreateInfo {
    /// The second defined general physical device features structure
    PhysicalDeviceFeatures2,

    /// Vulkan 1.3 specific features
    PhysicalDeviceVulkan13Features,

    /// Dynamic rendering features
    PhysicalDeviceExtendedDynamicStateFeatures,
}
