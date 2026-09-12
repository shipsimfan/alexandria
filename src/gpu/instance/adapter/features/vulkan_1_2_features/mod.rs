use crate::gpu::VulkanAdapterFeature;
use vulkan::VkPhysicalDeviceVulkan12Features;

mod get;
mod next_chain;
mod set;

/// Vulkan 1.2 device features
#[derive(Default, Clone)]
pub struct VulkanDeviceVulkan12Features {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceVulkan12Features,
}

impl VulkanAdapterFeature for VulkanDeviceVulkan12Features {}
