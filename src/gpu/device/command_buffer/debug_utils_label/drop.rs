use crate::gpu::VulkanDebugUtilsLabel;

impl<'a> Drop for VulkanDebugUtilsLabel<'a> {
    fn drop(&mut self) {
        unsafe {
            (self
                .command_buffer
                .device
                .functions()
                .command_buffer
                .debug_utils()
                .cmd_end_debug_utils_label)(self.command_buffer.handle)
        }
    }
}
