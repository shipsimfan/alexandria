use crate::{GraphicsInstance, GraphicsInstanceBuilder, GraphicsVersion};

impl<'a> GraphicsInstanceBuilder<'a> {
    /// Create a new [`GraphicsInstanceBuilder`]
    pub fn new(api_version: GraphicsVersion) -> GraphicsInstanceBuilder<'a> {
        GraphicsInstanceBuilder {
            api_version,
            application: None,
            engine: None,
            layers: Vec::new(),
        }
    }
}

impl GraphicsInstance {
    /// Create a new [`GraphicsInstanceBuilder`]
    pub fn builder<'a>(api_version: GraphicsVersion) -> GraphicsInstanceBuilder<'a> {
        GraphicsInstanceBuilder::new(api_version)
    }
}
