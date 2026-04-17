use crate::gpu::VulkanCommandBuffer;

impl Drop for VulkanCommandBuffer {
    fn drop(&mut self) {
        self.command_pool
            .with_handle_and_device(|command_pool, device| unsafe {
                (device.functions().command_buffer.free_command_buffers)(
                    device.handle(),
                    command_pool,
                    1,
                    &self.handle,
                )
            })
    }
}
