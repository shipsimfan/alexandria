use crate::gpu::{VulkanCommandBuffer, VulkanViewport};

impl VulkanCommandBuffer {
    /// Set the viewport to use for subsequent draw calls in the command buffer
    pub fn cmd_set_viewport(&mut self, first_viewport: u32, viewports: &[VulkanViewport]) {
        unsafe {
            (self.device.functions().command_buffer.cmd_set_viewport)(
                self.handle,
                first_viewport,
                viewports.len() as u32,
                viewports.as_ptr().cast(),
            )
        };
    }
}
