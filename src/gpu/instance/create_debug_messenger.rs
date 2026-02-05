use crate::{
    Result,
    gpu::{
        VulkanDebugMessageSeverity, VulkanDebugMessenger, VulkanDebugMessengerCallback,
        VulkanInstance,
    },
};

impl VulkanInstance {
    /// Create a new [`VulkanDebugMessenger`]
    pub fn create_debug_messenger<C: VulkanDebugMessengerCallback>(
        &self,
        min_severity: VulkanDebugMessageSeverity,
        callback: C,
    ) -> Result<VulkanDebugMessenger<C>> {
        self.inner
            .clone()
            .create_debug_messenger(min_severity, callback)
    }
}
