use crate::{
    GraphicsInstanceLayer, Result,
    instance::{GraphicsInstanceInner, inner::GraphicsInstanceFunctions},
};
use vulkan::VkInstance;

impl GraphicsInstanceInner {
    /// Create a new [`GraphicsInstanceInner`]
    pub(in crate::instance) fn new(
        handle: VkInstance,
        layers: &[GraphicsInstanceLayer],
    ) -> Result<GraphicsInstanceInner> {
        let functions = GraphicsInstanceFunctions::load(handle, layers)?;

        Ok(GraphicsInstanceInner { handle, functions })
    }
}
