use vulkan::VkBufferCopy;

mod get;
mod new;
mod set;

/// The description of a region of a buffer to copy
#[repr(transparent)]
pub struct VulkanBufferCopy {
    /// The inner Vulkan structure representing the buffer copy region
    inner: VkBufferCopy,
}
