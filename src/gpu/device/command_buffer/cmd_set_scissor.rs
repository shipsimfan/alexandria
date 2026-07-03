use crate::{gpu::VulkanCommandBuffer, math::Recti};

impl VulkanCommandBuffer {
    /// Set the scissor rects to use for subsequent draw calls in the command buffer
    pub fn cmd_set_scissor(&mut self, first_scissor: u32, scissors: &[Recti]) {
        unsafe {
            (self.device.functions().command_buffer.cmd_set_scissor)(
                self.handle,
                first_scissor,
                scissors.len() as u32,
                scissors.as_ptr().cast(),
            )
        };
    }
}
