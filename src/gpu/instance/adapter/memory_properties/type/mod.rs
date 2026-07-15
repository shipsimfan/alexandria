use vulkan::VkMemoryType;

mod get;

/// The description of a Vulkan memory type
#[repr(transparent)]
#[derive(Debug)]
pub struct VulkanMemoryType {
    /// The raw description of the Vulkan memory type
    inner: VkMemoryType,
}
