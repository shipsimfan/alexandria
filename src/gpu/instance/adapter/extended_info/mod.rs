mod extended_dynamic_state_features;
mod features;
mod vulkan_1_3_features;

mod as_ptr;
mod from;
mod into;
mod set_next;

pub use extended_dynamic_state_features::*;
pub use features::*;
pub use vulkan_1_3_features::*;

/// Extended information about a Vulkan adapter that can be provided when creating a graphics
/// device
#[derive(Clone)]
pub enum VulkanExtendedAdapterInfo {
    /// The general device features
    Features(VulkanDeviceFeatures),

    /// Vulkan 1.3 specific features
    Vulkan13Features(VulkanDeviceVulkan13Features),

    /// Extended dynamic rendering features
    ExtendedDynamicStateFeatures(VulkanDeviceExtendedDynamicStateFeatures),
}
