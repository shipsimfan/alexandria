use vulkan::VkBuffer;

mod get;

/// A Vulkan buffer object
pub struct VulkanBuffer {
    /// The handle to the underlying Vulkan buffer
    handle: VkBuffer,
}
