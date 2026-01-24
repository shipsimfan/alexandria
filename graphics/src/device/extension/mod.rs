mod display;
mod strs;

/// Vulkan instance extensions that are supported by Alexandria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsDeviceExtension {
    /// The extension to enable using swapchains
    Swapchain,

    /// The extension to enable extended dynamic rendering
    ExtendedDynamicState,
}
