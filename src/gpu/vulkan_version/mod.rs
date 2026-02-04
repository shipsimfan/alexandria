mod constants;
mod display;
mod get;
mod new;
mod supports;

/// A version as used in Vulkan
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VulkanVersion {
    /// The encoded version
    version: u32,
}
