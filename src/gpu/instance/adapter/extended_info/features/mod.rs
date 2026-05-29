use vulkan::VkPhysicalDeviceFeatures2;

mod as_ptr;
mod set_next;

/// General Vulkan device features
#[derive(Default, Clone)]
pub struct VulkanDeviceFeatures {
    /// The inner Vulkan structure that this wraps
    inner: VkPhysicalDeviceFeatures2,
}
