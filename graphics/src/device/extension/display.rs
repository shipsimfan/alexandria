use crate::GraphicsDeviceExtension;

impl std::fmt::Display for GraphicsDeviceExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsDeviceExtension::Swapchain => write!(f, "swapchain"),
        }
    }
}
