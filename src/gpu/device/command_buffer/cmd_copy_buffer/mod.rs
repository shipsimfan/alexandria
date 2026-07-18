use crate::gpu::{VulkanBuffer, VulkanCommandBuffer};

mod buffer_copy;

pub use buffer_copy::*;

impl VulkanCommandBuffer {
    /// Copy data from one buffer to another
    pub fn cmd_copy_buffer(
        &mut self,
        src_buffer: &VulkanBuffer,
        dst_buffer: &VulkanBuffer,
        regions: &[VulkanBufferCopy],
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_copy_buffer)(
                self.handle,
                src_buffer.handle(),
                dst_buffer.handle(),
                regions.len() as u32,
                regions.as_ptr().cast(),
            )
        };
    }
}
