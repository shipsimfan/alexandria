use crate::gpu::{VulkanCommandBuffer, VulkanDependencyFlags};
use vulkan::VkDependencyInfo;

mod buffer_memory_barrier;
mod image_memory_barrier;
mod memory_barrier;

pub use buffer_memory_barrier::*;
pub use image_memory_barrier::*;
pub use memory_barrier::*;

impl VulkanCommandBuffer {
    /// Insert a pipeline barrier into the command buffer to transition an image between layouts
    pub fn cmd_pipeline_barrier2<F: Into<VulkanDependencyFlags>>(
        &mut self,
        flags: F,
        memory_barriers: &[VulkanMemoryBarrier],
        buffer_memory_barriers: &[VulkanBufferMemoryBarrier],
        image_memory_barriers: &[VulkanImageMemoryBarrier],
    ) {
        let dependency_info = VkDependencyInfo {
            dependency_flags: flags.into(),
            memory_barrier_count: memory_barriers.len() as u32,
            memory_barriers: memory_barriers.as_ptr().cast(),
            buffer_memory_barrier_count: buffer_memory_barriers.len() as u32,
            buffer_memory_barriers: buffer_memory_barriers.as_ptr().cast(),
            image_memory_barrier_count: image_memory_barriers.len() as u32,
            image_memory_barriers: image_memory_barriers.as_ptr().cast(),
            ..Default::default()
        };

        unsafe {
            (self.device.functions().command_buffer.cmd_pipeline_barrier2)(
                self.handle,
                &dependency_info,
            )
        };
    }
}
