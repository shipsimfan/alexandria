use crate::gpu::VulkanDebugMessageSeverity;

impl std::fmt::Display for VulkanDebugMessageSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
