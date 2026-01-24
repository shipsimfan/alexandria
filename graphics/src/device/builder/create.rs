use crate::{GraphicsDevice, GraphicsDeviceBuilder, Result, device::inner::GraphicsDeviceInner};
use std::sync::Arc;

impl<'adapter, 'instance, 'a> GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`GraphicsDevice`] with the provided settings
    pub fn create(&self) -> Result<GraphicsDevice> {
        Ok(GraphicsDevice {
            inner: Arc::new(GraphicsDeviceInner::new(
                &self.extended_info,
                &self.queues,
                &self.extensions,
                self.adapter,
            )?),
        })
    }
}
