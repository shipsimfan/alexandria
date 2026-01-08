mod display;
mod strs;

/// Vulkan instance extensions that are supported by Alexandria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsInstanceExtension {
    /// The debug utilities extension
    DebugUtils,
}
