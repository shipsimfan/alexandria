mod display;
mod strs;

/// Vulkan instance layers that are supported by Alexandria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsInstanceLayer {
    /// The Khronos validation layer
    KhronosValidation,
}
