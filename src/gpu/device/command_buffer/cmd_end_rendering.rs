use crate::gpu::VulkanCommandBuffer;

impl VulkanCommandBuffer {
    /// End the current dynamic rendering pass
    pub fn cmd_end_rendering(&mut self) {
        unsafe { (self.device.functions().command_buffer.cmd_end_rendering)(self.handle) };
    }
}
