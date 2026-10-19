use vulkan::VkFramebuffer;

mod get;
mod nameable;

/// A Vulkan framebuffer represents a collection of attachments that are used during rendering
pub struct VulkanFramebuffer {
    /// The handle to the underlying Vulkan framebuffer
    handle: VkFramebuffer,
}
