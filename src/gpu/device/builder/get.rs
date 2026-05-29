use crate::gpu::{
    VulkanDeviceBuilder, VulkanDeviceExtension, VulkanExtendedAdapterInfo, VulkanQueueCreateInfo,
};

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Get the extended information adjusting the device to be created
    pub fn get_extended_info(&self) -> &[VulkanExtendedAdapterInfo] {
        &self.extended_info
    }

    /// Get the information describing the queues to be created
    pub fn get_queues(&self) -> &[VulkanQueueCreateInfo<'a>] {
        &self.queues
    }

    /// Get the extensions that have been requested for this device to be created
    pub fn get_extensions(&self) -> &[VulkanDeviceExtension] {
        &self.extensions
    }
}
