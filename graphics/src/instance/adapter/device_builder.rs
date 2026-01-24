use crate::{GraphicsAdapter, GraphicsDeviceBuilder};

impl<'instance> GraphicsAdapter<'instance> {
    /// Create a new [`GraphicsDeviceBuilder`]
    pub fn device_builder<'adapter, 'a>(
        &'adapter self,
    ) -> GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
        GraphicsDeviceBuilder::new(self)
    }
}
