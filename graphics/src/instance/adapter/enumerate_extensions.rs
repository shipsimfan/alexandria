use crate::{GraphicsAdapter, GraphicsDeviceExtension, Result};

impl<'instance> GraphicsAdapter<'instance> {
    /// Enumerate all Alexandria supported Vulkan extensions supported on this adapter
    pub fn enumerate_extensions(&self) -> Result<Vec<GraphicsDeviceExtension>> {
        Ok(self
            .enumerate_all_extensions()?
            .into_iter()
            .filter_map(|extension| GraphicsDeviceExtension::from_str(extension.name()))
            .collect())
    }
}
