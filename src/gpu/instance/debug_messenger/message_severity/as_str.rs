use crate::gpu::VulkanDebugMessageSeverity;

impl VulkanDebugMessageSeverity {
    /// Get a string representing this severity
    pub const fn as_str(&self) -> &'static str {
        match self {
            VulkanDebugMessageSeverity::Verbose => "verbose",
            VulkanDebugMessageSeverity::Info => "info",
            VulkanDebugMessageSeverity::Warning => "warning",
            VulkanDebugMessageSeverity::Error => "error",
        }
    }
}
