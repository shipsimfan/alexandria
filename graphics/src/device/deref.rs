use crate::{GraphicsDevice, device::GraphicsDeviceInner};
use std::{ops::Deref, sync::Arc};

impl Deref for GraphicsDevice {
    type Target = Arc<GraphicsDeviceInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
