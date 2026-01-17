use crate::{GraphicsDeviceBuilder, GraphicsInstance};

impl<'instance, 'a> GraphicsDeviceBuilder<'instance, 'a> {
    /// Create a new [`GraphicsDeviceBuilder`]
    pub(crate) fn new(
        instance: &'instance GraphicsInstance,
    ) -> GraphicsDeviceBuilder<'instance, 'a> {
        GraphicsDeviceBuilder {
            extended_info: Vec::new(),
            queues: Vec::new(),
            extensions: Vec::new(),
            instance,
        }
    }
}
