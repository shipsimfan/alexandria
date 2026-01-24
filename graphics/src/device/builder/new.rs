use crate::{GraphicsAdapter, GraphicsDeviceBuilder};

impl<'adapter, 'instance, 'a> GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
    /// Create a new [`GraphicsDeviceBuilder`]
    pub(crate) fn new(
        adapter: &'adapter GraphicsAdapter<'instance>,
    ) -> GraphicsDeviceBuilder<'adapter, 'instance, 'a> {
        GraphicsDeviceBuilder {
            extended_info: Vec::new(),
            queues: Vec::new(),
            extensions: Vec::new(),
            adapter,
        }
    }
}
