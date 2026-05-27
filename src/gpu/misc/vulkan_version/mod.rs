mod r#macro;

mod constants;
mod display;
mod from_str;
mod from_strs;
mod get;
mod new;
mod supports;

pub use from_str::*;

/// A version as used in Vulkan
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VulkanVersion {
    /// The encoded version
    version: u32,
}
