use crate::GraphicsVersion;

mod display;
mod eq;
mod get;
mod new;

/// A raw view of an layer provided by the [`GraphicsInstance`](crate::GraphicsInstance) or
/// [`GraphicsDevice`](crate::GraphicsDevice)
#[derive(Debug, Clone, Hash)]
pub struct GraphicsLayer {
    /// The name of the layer
    name: String,

    /// A description of the layer
    description: String,

    /// The specification version of the layer implemented
    spec_version: GraphicsVersion,

    /// The driver reported version of the layer
    version: GraphicsVersion,
}
