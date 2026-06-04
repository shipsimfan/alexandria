use crate::gpu::{
    VulkanAdapterFeature, VulkanDeviceBuilder, VulkanDeviceExtension, VulkanQueueCreateInfo,
};

impl<'adapter, 'instance, 'a> VulkanDeviceBuilder<'adapter, 'instance, 'a> {
    /// Get the features adjusting the device to be created
    pub fn get_features(&self) -> &[&'a mut dyn VulkanAdapterFeature] {
        &self.features
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
