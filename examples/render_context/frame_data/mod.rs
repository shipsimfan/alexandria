use alexandria::gpu::{VulkanCommandBuffer, VulkanFence, VulkanSemaphore};

mod new;

/// Data associated with a single frame in flight
pub(in crate::render_context) struct FrameData {
    /// The buffer to record rendering commands into for this frame
    pub command_buffer: VulkanCommandBuffer,

    /// Semaphore signaled when the GPU has finished rendering this frame and presentation can begin
    pub render_complete_semaphore: VulkanSemaphore,

    /// Semaphore signaled when the presentation engine has finished reading from the swapchain image
    pub present_complete_semaphore: VulkanSemaphore,

    /// Fence signaled when the GPU has finished executing the command buffer for this frame
    pub draw_fence: VulkanFence,
}
