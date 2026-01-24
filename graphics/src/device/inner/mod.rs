use crate::GraphicsInstance;
use functions::GraphicsDeviceFunctions;
use vulkan::VkDevice;

mod functions;

mod drop;
mod new;

/// An interface for interacting with a specific Vulkan device
pub struct GraphicsDeviceInner {
    /// A handle to the underlying graphics device
    handle: VkDevice,

    /// A reference to the instance that produced this device
    _instance: GraphicsInstance,

    /// The functions loaded for this device
    pub(in crate::device) functions: GraphicsDeviceFunctions,
}
