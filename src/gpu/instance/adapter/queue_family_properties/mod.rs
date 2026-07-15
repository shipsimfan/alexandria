use vulkan::VkQueueFamilyProperties;

mod get;

/// Information describing a queue family
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VulkanQueueFamilyProperties {
    /// The raw Vulkan queue family properties
    inner: VkQueueFamilyProperties,
}
