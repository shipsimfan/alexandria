use crate::gpu::VulkanViewport;
use vulkan::VkViewport;

impl VulkanViewport {
    /// Create a new [`VulkanViewport`]
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> VulkanViewport {
        VulkanViewport {
            inner: VkViewport {
                x,
                y,
                width,
                height,
                min_depth,
                max_depth,
            },
        }
    }
}
