use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceFeatures2;

mod next_chain;

/// General Vulkan device features
#[derive(Default, Clone)]
pub struct VulkanDeviceFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceFeatures2,
}

impl VulkanAdapterFeature for VulkanDeviceFeatures {}
