use functions::GraphicsDeviceFunctions;
use vulkan::VkDevice;

mod functions;

/// An interface for interacting with a specific Vulkan device
pub struct GraphicsDeviceInner {
    /// A handle to the underlying graphics device
    handle: VkDevice,

    /// The functions loaded for this device
    pub(in crate::device) functions: GraphicsDeviceFunctions,
}
