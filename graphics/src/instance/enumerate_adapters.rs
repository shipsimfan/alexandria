use crate::{GraphicsAdapter, GraphicsInstance, Result};

impl GraphicsInstance {
    /// Enumerate all the [`GraphicsAdapter`]s on the system
    pub fn enumerate_adapters<'instance>(
        &'instance self,
    ) -> Result<Vec<GraphicsAdapter<'instance>>> {
        self.inner.enumerate_adapters(self)
    }
}
