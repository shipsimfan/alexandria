use vulkan::VkRenderPass;

mod get;
mod nameable;

/// A collection of attachments, subpasses, and dependencies that define how rendering is performed
pub struct VulkanRenderPass {
    /// The handle to the underlying Vulkan render pass
    handle: VkRenderPass,
}
