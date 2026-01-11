mod display;
mod strs;

/// Vulkan instance extensions that are supported by Alexandria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsInstanceExtension {
    /// The debug utilities extension
    DebugUtils,

    /// General window surfaces extension
    Surface,

    /// Windows-specific window surface extension
    #[cfg(target_os = "windows")]
    Win32Surface,
}
