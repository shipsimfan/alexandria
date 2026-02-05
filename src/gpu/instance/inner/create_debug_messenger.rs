use crate::{
    Result,
    gpu::{
        VulkanDebugMessageSeverity, VulkanDebugMessenger, VulkanDebugMessengerCallback,
        instance::VulkanInstanceInner,
    },
};
use std::sync::Arc;

impl VulkanInstanceInner {
    /// Create a new [`VulkanDebugMessenger`]
    pub(in crate::gpu::instance) fn create_debug_messenger<C: VulkanDebugMessengerCallback>(
        self: Arc<Self>,
        min_severity: VulkanDebugMessageSeverity,
        callback: C,
    ) -> Result<VulkanDebugMessenger<C>> {
        VulkanDebugMessenger::new(self, min_severity, callback)
    }
}
