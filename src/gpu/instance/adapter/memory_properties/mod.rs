use vulkan::VkPhysicalDeviceMemoryProperties;

mod heap;
mod r#type;

mod get;
mod new;

pub use heap::*;
pub use r#type::*;

/// Information describing the memory properties of a Vulkan adapter
#[derive(Debug)]
pub struct VulkanAdapterMemoryProperties {
    /// The raw Vulkan adapter memory properties
    inner: VkPhysicalDeviceMemoryProperties,
}
