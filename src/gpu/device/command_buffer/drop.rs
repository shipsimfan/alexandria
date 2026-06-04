use crate::gpu::VulkanCommandBuffer;

impl Drop for VulkanCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().command_buffer.free_command_buffers)(
                self.command_pool.device().handle(),
                self.command_pool.handle(),
                1,
                &self.handle,
            )
        }
    }
}
