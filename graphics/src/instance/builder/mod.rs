use crate::{GraphicsInstanceLayer, GraphicsVersion};
use std::borrow::Cow;

mod create;
mod get;
mod new;
mod set;

/// A builder for [`GraphicsInstance`](crate::GraphicsInstance)s
pub struct GraphicsInstanceBuilder<'a> {
    /// The requested version of Vulkan to use
    api_version: GraphicsVersion,

    /// The name and version of the application being run
    application: Option<(Cow<'a, str>, GraphicsVersion)>,

    /// The name and version of the engine being run
    engine: Option<(Cow<'a, str>, GraphicsVersion)>,

    /// The requested layers to enable for the instance
    layers: Vec<GraphicsInstanceLayer>,
}
