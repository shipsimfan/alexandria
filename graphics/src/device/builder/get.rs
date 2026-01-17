use crate::{
    GraphicsDeviceBuilder, GraphicsDeviceExtendedCreateInfo, GraphicsDeviceExtension,
    GraphicsQueueCreateInfo,
};

impl<'instance, 'a> GraphicsDeviceBuilder<'instance, 'a> {
    /// Get the extended information adjusting the device to be created
    pub fn get_extended_info(&self) -> &[GraphicsDeviceExtendedCreateInfo] {
        &self.extended_info
    }

    /// Get the information describing the queues to be created
    pub fn get_queues(&self) -> &[GraphicsQueueCreateInfo<'a>] {
        &self.queues
    }

    /// Get the extensions that have been requested for this device to be created
    pub fn get_extensions(&self) -> &[GraphicsDeviceExtension] {
        &self.extensions
    }
}
