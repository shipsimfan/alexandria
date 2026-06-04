use alexandria::gpu::{VulkanFence, VulkanSemaphore};

mod new;

/// Data associated with a single frame in flight
pub(in crate::render_context) struct FrameData {
    /// Semaphore signaled when the GPU has finished rendering this frame and presentation can begin
    pub render_complete_semaphore: VulkanSemaphore,

    /// Semaphore signaled when the presentation engine has finished reading from the swapchain image
    pub acquire_image_semaphore: VulkanSemaphore,

    /// Fence signaled when the GPU has finished executing the command buffer for this frame
    pub draw_fence: VulkanFence,
}
