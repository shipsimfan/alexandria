use crate::{
    Error, Result,
    gpu::{VulkanCommandBuffer, VulkanFence, VulkanQueue, VulkanSemaphore},
};
use vulkan::{VkPipelineStageFlag, VkSubmitInfo, try_vulkan};

impl VulkanQueue {
    /// Submits a command buffer to this queue, with the given wait and signal semaphores
    pub fn submit(
        &mut self,
        command_buffer: &VulkanCommandBuffer,
        wait_semaphores: &VulkanSemaphore,
        signal_semaphores: &VulkanSemaphore,
        fence: &mut VulkanFence,
    ) -> Result<()> {
        let submit_info = VkSubmitInfo {
            wait_semaphore_count: 1,
            wait_semaphores: &wait_semaphores.handle(),
            wait_dst_stage_mask: &VkPipelineStageFlag::ColorAttachmentOutput.into(),
            command_buffer_count: 1,
            command_buffers: &command_buffer.handle(),
            signal_semaphore_count: 1,
            signal_semaphores: &signal_semaphores.handle(),
            ..Default::default()
        };

        try_vulkan!((self.device.functions().queue.submit)(
            self.handle,
            1,
            &submit_info,
            fence.handle(),
        ))
        .map_err(|vk| Error::new_with("unable to submit queue", vk))?;

        Ok(())
    }
}
