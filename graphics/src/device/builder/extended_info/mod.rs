mod extended_dynamic_state_features;
mod features;
mod vk;
mod vulkan_1_3_features;

mod from;
mod to_vk;

pub use extended_dynamic_state_features::GraphicsDeviceExtendedDynamicStateFeatures;
pub use features::GraphicsDeviceFeatures;
pub use vulkan_1_3_features::GraphicsDeviceVulkan13Features;

pub(in crate::device) use vk::VkGraphicsDeviceExtendedCreateInfo;

/// Extended information that can be provided when creating a graphics device
pub enum GraphicsDeviceExtendedCreateInfo {
    /// The general device features
    Features(GraphicsDeviceFeatures),

    /// Vulkan 1.3 specific features
    Vulkan13Features(GraphicsDeviceVulkan13Features),

    /// Extended dynamic rendering features
    ExtendedDynamicStateFeatures(GraphicsDeviceExtendedDynamicStateFeatures),
}
