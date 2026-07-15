use vulkan::VkPhysicalDeviceMemoryProperties;

mod heap;
mod r#type;

pub use heap::*;
pub use r#type::*;

/// Information describing the memory properties of a Vulkan adapter
#[repr(transparent)]
#[derive(Debug)]
pub struct VulkanAdapterMemoryProperties {
    /// The raw Vulkan adapter memory properties
    inner: VkPhysicalDeviceMemoryProperties,
}
