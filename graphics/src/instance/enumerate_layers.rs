use crate::{GraphicsInstance, GraphicsInstanceLayer, Result};

impl GraphicsInstance {
    /// Enumerate all Alexandria supported Vulkan layers supported on this system
    pub fn enumerate_layers() -> Result<Vec<GraphicsInstanceLayer>> {
        Ok(GraphicsInstance::enumerate_all_layers()?
            .into_iter()
            .filter_map(|layer| GraphicsInstanceLayer::from_str(layer.name()))
            .collect())
    }
}
