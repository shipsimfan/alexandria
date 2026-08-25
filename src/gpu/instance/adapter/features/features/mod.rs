use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceFeatures2;

mod get;
mod next_chain;
mod set;

/// General Vulkan device features
#[derive(Default, Clone)]
pub struct VulkanDeviceFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceFeatures2,
}

impl VulkanAdapterFeature for VulkanDeviceFeatures {}
