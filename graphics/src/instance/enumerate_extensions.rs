use crate::{GraphicsInstance, GraphicsInstanceExtension, Result};

impl GraphicsInstance {
    /// Enumerate all Alexandria supported Vulkan extensions supported on this system
    pub fn enumerate_extensions() -> Result<Vec<GraphicsInstanceExtension>> {
        Ok(GraphicsInstance::enumerate_all_extensions(None)?
            .into_iter()
            .filter_map(|extension| GraphicsInstanceExtension::from_str(extension.name()))
            .collect())
    }
}
