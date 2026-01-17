use inner::GraphicsDeviceInner;
use std::sync::Arc;

mod builder;
mod extension;
mod inner;
mod swapchain;

mod deref;

pub use builder::*;
pub use extension::GraphicsDeviceExtension;
pub use swapchain::*;

/// An interface for interacting with a specific Vulkan device
pub struct GraphicsDevice {
    /// Reference to the graphics device
    inner: Arc<GraphicsDeviceInner>,
}
