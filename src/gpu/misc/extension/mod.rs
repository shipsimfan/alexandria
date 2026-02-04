use crate::gpu::VulkanVersion;

mod display;
mod get;
mod new;

/// A raw view of an extension provided by the [`GraphicsInstance`](crate::GraphicsInstance) or
/// [`GraphicsDevice`](crate::GraphicsDevice)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanExtension {
    /// The name of the extension
    name: String,

    /// The version of the extension provided
    version: VulkanVersion,
}
