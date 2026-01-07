use crate::GraphicsDebugMessageSeverity;

impl GraphicsDebugMessageSeverity {
    /// Get a string representing this severity
    pub const fn as_str(&self) -> &'static str {
        match self {
            GraphicsDebugMessageSeverity::Verbose => "verbose",
            GraphicsDebugMessageSeverity::Info => "info",
            GraphicsDebugMessageSeverity::Warning => "warning",
            GraphicsDebugMessageSeverity::Error => "error",
        }
    }
}
