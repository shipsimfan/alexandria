use crate::gpu::device::{VulkanCommandBufferDebugUtilFunctions, VulkanCommandBufferFunctions};

impl VulkanCommandBufferFunctions {
    /// Get a reference to the debug util functions for a command buffer
    pub fn debug_utils(&self) -> &VulkanCommandBufferDebugUtilFunctions {
        self.debug_utils.as_ref().unwrap()
    }
}
