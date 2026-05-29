use vulkan::VkPhysicalDeviceVulkan13Features;

mod as_ptr;
mod get;
mod set;
mod set_next;

/// Vulkan 1.3 device features
#[derive(Default, Clone)]
pub struct VulkanDeviceVulkan13Features {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceVulkan13Features,
}
