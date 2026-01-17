use crate::{GraphicsDevice, device::GraphicsDeviceInner};
use std::ops::Deref;

impl Deref for GraphicsDevice {
    type Target = GraphicsDeviceInner;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
