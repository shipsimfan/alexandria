use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceVulkan13Features;

mod get;
mod next_chain;
mod set;

/// Vulkan 1.3 device features
#[derive(Default, Clone)]
pub struct VulkanDeviceVulkan13Features {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceVulkan13Features,
}

impl VulkanAdapterFeature for VulkanDeviceVulkan13Features {}
