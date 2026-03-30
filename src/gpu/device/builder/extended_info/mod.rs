mod extended_dynamic_state_features;
mod features;
mod vk;
mod vulkan_1_3_features;

mod from;
mod to_vk;

pub use extended_dynamic_state_features::*;
pub use features::*;
pub use vulkan_1_3_features::*;

pub(in crate::gpu::device) use vk::VkVulkanDeviceExtendedCreateInfo;

/// Extended information that can be provided when creating a graphics device
pub enum VulkanDeviceExtendedCreateInfo {
    /// The general device features
    Features(VulkanDeviceFeatures),

    /// Vulkan 1.3 specific features
    Vulkan13Features(VulkanDeviceVulkan13Features),

    /// Extended dynamic rendering features
    ExtendedDynamicStateFeatures(VulkanDeviceExtendedDynamicStateFeatures),
}
