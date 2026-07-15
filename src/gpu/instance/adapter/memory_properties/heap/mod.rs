use vulkan::VkMemoryHeap;

mod get;

/// The properties of a Vulkan memory heap
#[repr(transparent)]
#[derive(Debug)]
pub struct VulkanMemoryHeap {
    /// The raw Vulkan properties of the memory heap
    inner: VkMemoryHeap,
}
