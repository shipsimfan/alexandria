use crate::GraphicsDebugMessageSeverity;

impl std::fmt::Display for GraphicsDebugMessageSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
