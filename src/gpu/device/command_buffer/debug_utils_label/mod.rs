use crate::gpu::VulkanCommandBuffer;

mod deref;
mod drop;
mod get;
mod new;

/// A label used for debugging Vulkan command buffers
pub struct VulkanDebugUtilsLabel<'a> {
    /// The command buffer this label is associated with
    command_buffer: &'a mut VulkanCommandBuffer,
}
