use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceVulkan11Features;

mod get;
mod next_chain;
mod set;

/// Vulkan 1.1 device features
#[derive(Default, Clone)]
pub struct VulkanDeviceVulkan11Features {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceVulkan11Features,
}

impl VulkanAdapterFeature for VulkanDeviceVulkan11Features {}
