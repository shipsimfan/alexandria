use crate::gpu::{VulkanInstanceFunctions, instance::VulkanDebugMessengerFunctions};

impl VulkanInstanceFunctions {
    /// Get the functions for debug messengers
    pub(in crate::gpu::instance) fn debug_messenger(&self) -> &VulkanDebugMessengerFunctions {
        self.debug_messenger.as_ref().unwrap()
    }
}
