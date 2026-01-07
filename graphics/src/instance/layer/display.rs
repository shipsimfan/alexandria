use crate::GraphicsInstanceLayer;

impl std::fmt::Display for GraphicsInstanceLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsInstanceLayer::KhronosValidation => write!(f, "Khronos validation"),
        }
    }
}
