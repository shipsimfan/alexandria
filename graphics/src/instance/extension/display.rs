use crate::GraphicsInstanceExtension;

impl std::fmt::Display for GraphicsInstanceExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsInstanceExtension::DebugUtils => write!(f, "debug utils"),
        }
    }
}
