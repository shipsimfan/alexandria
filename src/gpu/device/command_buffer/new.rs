use crate::{
    Error, Result,
    gpu::{VulkanCommandBuffer, VulkanCommandBufferLevel, VulkanCommandPool},
};
use vulkan::{VkCommandBuffer, VkCommandBufferAllocateInfo, try_vulkan};

impl VulkanCommandBuffer {
    /// Create a new [`VulkanCommandBuffer`]
    pub(in crate::gpu::device) fn new(
        command_pool: &mut VulkanCommandPool,
        level: VulkanCommandBufferLevel,
    ) -> Result<VulkanCommandBuffer> {
        let allocate_info = VkCommandBufferAllocateInfo {
            command_pool: command_pool.handle(),
            level,
            command_buffer_count: 1,
            ..Default::default()
        };

        let mut handle = VkCommandBuffer::null();
        try_vulkan!((command_pool
            .device()
            .functions()
            .command_buffer
            .allocate_command_buffers)(
            command_pool.device().handle(),
            &allocate_info,
            &mut handle,
        ))
        .map_err(|vk| Error::new_with("unable to allocate command buffer", vk))?;

        Ok(VulkanCommandBuffer {
            handle,
            device: command_pool.device().clone(),
        })
    }
}
