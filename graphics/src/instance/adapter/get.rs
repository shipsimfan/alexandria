use crate::{GraphicsAdapter, GraphicsAdapterKind, GraphicsVersion};

impl<'instance> GraphicsAdapter<'instance> {
    /// Get the Vulkan version supported by this adapter
    pub fn api_version(&self) -> GraphicsVersion {
        self.api_version
    }

    /// Get the version of the driver this adapter uses
    pub fn driver_version(&self) -> GraphicsVersion {
        self.driver_version
    }

    /// Get the kind of graphics adapter this is
    pub fn kind(&self) -> GraphicsAdapterKind {
        self.kind
    }

    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }
}
