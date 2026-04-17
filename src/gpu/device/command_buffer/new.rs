use crate::{
    Error, Result,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};
use vulkan::{VkCommandBuffer, VkCommandBufferAllocateInfo, try_vulkan};

impl VulkanCommandBuffer {
    /// Create a new [`VulkanCommandBuffer`]
    pub(in crate::gpu::device) fn new(
        command_pool: VulkanCommandPool,
    ) -> Result<VulkanCommandBuffer> {
        let handle = command_pool.with_handle_and_device(|command_pool, device| {
            let allocate_info = VkCommandBufferAllocateInfo {
                command_pool,
                command_buffer_count: 1,
                ..Default::default()
            };

            let mut handle = VkCommandBuffer::null();
            try_vulkan!(
                (device.functions().command_buffer.allocate_command_buffers)(
                    device.handle(),
                    &allocate_info,
                    &mut handle,
                )
            )
            .map_err(|vk| Error::new_with("unable to allocate command buffer", vk))?;
            Ok(handle)
        })?;

        Ok(VulkanCommandBuffer {
            handle,
            command_pool,
        })
    }
}
