use vulkan::VkQueueFamilyProperties;

mod get;
mod new;

/// Information describing a queue family
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VulkanQueueFamilyProperties {
    /// The queue family index
    index: u32,

    /// The queue family properties
    properties: VkQueueFamilyProperties,
}
