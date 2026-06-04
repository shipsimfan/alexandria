use crate::{
    Result,
    gpu::{
        VulkanDebugMessageSeverityFlags, VulkanDebugMessageTypeFlags, VulkanDebugMessenger,
        VulkanDebugMessengerCallback, VulkanInstance,
    },
};

impl VulkanInstance {
    /// Create a new [`VulkanDebugMessenger`]
    pub fn create_debug_messenger<
        C: VulkanDebugMessengerCallback,
        S: Into<VulkanDebugMessageSeverityFlags>,
        T: Into<VulkanDebugMessageTypeFlags>,
    >(
        &self,
        message_severity: S,
        message_type: T,
        callback: C,
    ) -> Result<VulkanDebugMessenger<C>> {
        VulkanDebugMessenger::new(self, message_severity.into(), message_type.into(), callback)
    }
}
