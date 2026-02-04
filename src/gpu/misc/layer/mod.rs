use crate::gpu::VulkanVersion;

mod display;
mod eq;
mod get;
mod new;

/// A raw view of an layer provided by the [`GraphicsInstance`](crate::GraphicsInstance) or
/// [`GraphicsDevice`](crate::GraphicsDevice)
#[derive(Debug, Clone, Hash)]
pub struct VulkanLayer {
    /// The name of the layer
    name: String,

    /// A description of the layer
    description: String,

    /// The specification version of the layer implemented
    spec_version: VulkanVersion,

    /// The driver reported version of the layer
    version: VulkanVersion,
}
